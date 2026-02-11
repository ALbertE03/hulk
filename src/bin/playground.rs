use actix_web::{web, App, HttpServer, HttpResponse};
use actix_files::Files;
use serde::{Deserialize, Serialize};

use hulk_compiler::parser::Parser;
use hulk_compiler::macros::expand_macros;
use hulk_compiler::ast::optimize::optimize_program;
use hulk_compiler::ast::transform::transform_implicit_functors;
use hulk_compiler::codegen::{CodeGenerator, llvm_target::LlvmGenerator};

use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

// ─── Estructuras JSON ────────────────────────────────────────

#[derive(Deserialize)]
struct RunRequest {
    code: String,
}

#[derive(Serialize, Clone)]
struct RunResponse {
    success: bool,
    output: String,
    errors: String,
    llvm_ir: String,
    time_ms: u64,
}

// ─── Pipeline de compilación + ejecución ─────────────────────

fn compile_and_run(code: &str) -> RunResponse {
    let start = Instant::now();

    // 1) Parsing
    let mut parser = Parser::new(code);
    let program = match parser.parse_program() {
        Ok(p) => p,
        Err(e) => {
            return RunResponse {
                success: false,
                output: String::new(),
                errors: format!("❌ Error de parsing:\n{}", e),
                llvm_ir: String::new(),
                time_ms: start.elapsed().as_millis() as u64,
            };
        }
    };

    // 2) Expansión de macros
    let mut expanded = expand_macros(program);

    // 3) Transformación de implicit functors (ANTES del semantic check)
    // Usamos un contexto vacío solo para detectar protocolos del AST
    let temp_ctx = hulk_compiler::semantic::Context::new();
    transform_implicit_functors(&mut expanded, &temp_ctx);

    // 4) Chequeo semántico
    let context = match hulk_compiler::semantic::check_program(&expanded) {
        Ok(ctx) => ctx,
        Err(errors) => {
            let msg = errors
                .iter()
                .map(|e| format!("  • {:?}", e))
                .collect::<Vec<_>>()
                .join("\n");
            return RunResponse {
                success: false,
                output: String::new(),
                errors: format!("❌ Errores semánticos:\n{}", msg),
                llvm_ir: String::new(),
                time_ms: start.elapsed().as_millis() as u64,
            };
        }
    };

    // 5) Optimización
    let optimized = optimize_program(expanded);

    // 6) Generación LLVM IR
    let generator = LlvmGenerator;
    let llvm_code = generator.generate(&optimized, &context);

    // 7) Escribir IR a archivo temporal
    let temp_dir = std::env::temp_dir();
    let ll_path = temp_dir.join("hulk_playground.ll");
    let bin_path = temp_dir.join("hulk_playground_bin");

    if let Err(e) = std::fs::write(&ll_path, &llvm_code) {
        return RunResponse {
            success: false,
            output: String::new(),
            errors: format!("Error escribiendo IR: {}", e),
            llvm_ir: llvm_code,
            time_ms: start.elapsed().as_millis() as u64,
        };
    }

    //  Generar LLVM IR optimizado para mostrar
    let optimized_ll_path = temp_dir.join("hulk_playground_opt.ll");
    let opt_result = Command::new("clang")
        .args([
            "-S",                           // Generar assembly/IR
            "-emit-llvm",                   // Emitir LLVM IR
            "-O3",                          // Optimizaciones
            "-o",
            optimized_ll_path.to_str().unwrap(),
            ll_path.to_str().unwrap(),
            "-lm",
            "-Wno-override-module",
        ])
        .stderr(Stdio::piped())
        .output();

    // Leer el IR optimizado si se generó correctamente
    let display_ir = if let Ok(ref out) = opt_result {
        if out.status.success() {
            std::fs::read_to_string(&optimized_ll_path).unwrap_or(llvm_code.clone())
        } else {
            llvm_code.clone()
        }
    } else {
        llvm_code.clone()
    };

    // Compilar a binario ejecutable
    let clang = Command::new("clang")
        .args([
            "-O3",                          // Máxima optimización
            "-march=native",                // Optimizar para la CPU actual
            "-o",
            bin_path.to_str().unwrap(),
            ll_path.to_str().unwrap(),
            "-lm",
            "-Wno-override-module",
        ])
        .stderr(Stdio::piped())
        .output();

    match clang {
        Ok(out) if !out.status.success() => {
            return RunResponse {
                success: false,
                output: String::new(),
                errors: format!(
                    "❌ Error de clang:\n{}",
                    String::from_utf8_lossy(&out.stderr)
                ),
                llvm_ir: display_ir.clone(),
                time_ms: start.elapsed().as_millis() as u64,
            };
        }
        Err(e) => {
            return RunResponse {
                success: false,
                output: String::new(),
                errors: format!("No se encontró clang: {}", e),
                llvm_ir: display_ir.clone(),
                time_ms: start.elapsed().as_millis() as u64,
            };
        }
        _ => {}
    }

    //  Ejecutar con timeout de 10 segundos
    let bin_str = bin_path.to_str().unwrap().to_string();
    let (tx, rx) = std::sync::mpsc::channel();

    std::thread::spawn(move || {
        let result = Command::new(&bin_str).output();
        let _ = tx.send(result);
    });

    match rx.recv_timeout(Duration::from_secs(10)) {
        Ok(Ok(output)) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            RunResponse {
                success: output.status.success(),
                output: stdout,
                errors: if output.status.success() {
                    String::new()
                } else {
                    stderr
                },
                llvm_ir: display_ir.clone(),
                time_ms: start.elapsed().as_millis() as u64,
            }
        }
        Ok(Err(e)) => RunResponse {
            success: false,
            output: String::new(),
            errors: format!("Error de ejecución: {}", e),
            llvm_ir: display_ir.clone(),
            time_ms: start.elapsed().as_millis() as u64,
        },
        Err(_) => RunResponse {
            success: false,
            output: String::new(),
            errors: "⏱️ Timeout: la ejecución excedió 10 segundos.\n💡 Verifica que no tienes un bucle infinito.".to_string(),
            llvm_ir: display_ir,
            time_ms: start.elapsed().as_millis() as u64,
        },
    }
}

// ─── Handler HTTP ────────────────────────────────────────────

async fn handle_run(req: web::Json<RunRequest>) -> HttpResponse {
    let code = req.code.clone();
    match web::block(move || compile_and_run(&code)).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::InternalServerError().json(RunResponse {
            success: false,
            output: String::new(),
            errors: format!("Error interno: {}", e),
            llvm_ir: String::new(),
            time_ms: 0,
        }),
    }
}

// ─── Main ────────────────────────────────────────────────────

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;
    println!();
    println!("  \x1b[32;1m💚 HULK Playground\x1b[0m");
    println!("  ─────────────────────────────");
    println!("  Servidor corriendo en:");
    println!("  → \x1b[4mhttp://localhost:{}\x1b[0m", port);
    println!();

    HttpServer::new(|| {
        App::new()
            .route("/api/run", web::post().to(handle_run))
            .service(Files::new("/", "./web").index_file("index.html"))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
