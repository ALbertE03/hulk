use crate::ast::nodes::*;
use crate::semantic::Context;
use crate::utils::Spanned;
use std::collections::HashMap;


pub struct LlvmGenerator;

impl super::CodeGenerator for LlvmGenerator {
    fn generate(&self, program: &Program, context: &Context) -> String {
        let mut ctx = Ctx::new(context);

        // ── Pass 0: Collect inheritance hierarchy from declarations ──────
        //   Build a map  child -> (parent_name, parent_args)
        let mut parent_map: HashMap<String, String> = HashMap::new();
        let mut decl_map: HashMap<String, &TypeDecl> = HashMap::new();
        for decl in &program.declarations {
            if let Declaration::Type(td) = decl {
                decl_map.insert(td.name.clone(), td);
                if let Some(ref pi) = td.parent {
                    parent_map.insert(td.name.clone(), pi.name.clone());
                }
            }
        }

        // ── Pass 1: Emit classes in topo order (parent before child) ─────
        let ordered = topo_sort_classes(&decl_map, &parent_map);
        for name in &ordered {
            if let Some(td) = decl_map.get(name.as_str()) {
                emit_class(&mut ctx, td);
            }
        }

        // ── Pass 2: Emit top-level function declarations ────────────────
        for decl in &program.declarations {
            if let Declaration::Function(fd) = decl {
                emit_function(&mut ctx, fd);
            }
        }

        // ── Pass 3: Emit GC sweep helper ────────────────────────────────
        emit_gc_sweep_fn(&mut ctx);

        // ── Pass 4: Emit @main ──────────────────────────────────────────
        ctx.functions.push_str("define i32 @main() {\nentry:\n");
        let _val = gen_expr(&mut ctx, &program.expr);
        // Sweep all tracked allocations before exit
        ctx.emit("call void @__hulk_gc_sweep()");
        ctx.functions.push_str("  ret i32 0\n}\n");

        // Assemble final output
        format!("{}\n{}\n{}\n{}", ctx.preamble, ctx.globals, ctx.functions, ctx.lambda_defs)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Topological sort for class declarations (ensures parent emitted before child)
// ─────────────────────────────────────────────────────────────────────────────
fn topo_sort_classes(
    decl_map: &HashMap<String, &TypeDecl>,
    parent_map: &HashMap<String, String>,
) -> Vec<String> {
    let mut visited: HashMap<String, bool> = HashMap::new();
    let mut order: Vec<String> = Vec::new();

    fn visit(
        name: &str,
        parent_map: &HashMap<String, String>,
        decl_map: &HashMap<String, &TypeDecl>,
        visited: &mut HashMap<String, bool>,
        order: &mut Vec<String>,
    ) {
        if visited.contains_key(name) { return; }
        visited.insert(name.to_string(), true);
        if let Some(parent) = parent_map.get(name) {
            if decl_map.contains_key(parent.as_str()) {
                visit(parent, parent_map, decl_map, visited, order);
            }
        }
        order.push(name.to_string());
    }

    for name in decl_map.keys() {
        visit(name, parent_map, decl_map, &mut visited, &mut order);
    }
    order
}

// ─────────────────────────────────────────────────────────────────────────────
// Internal Context
// ─────────────────────────────────────────────────────────────────────────────

/// Full layout info for a class, including inherited attributes / methods.
#[allow(dead_code)]
struct ClassLayout {
    struct_name: String,                    // %T.MyClass
    type_id: u32,                           // unique integer id for is/as
    /// All attributes (parent first, then own), name -> (index, owning_class)
    attr_indices: HashMap<String, u32>,
    /// Ordered list of attribute names (for iteration)
    attr_order: Vec<String>,
    /// method_name -> @ClassName_method  (includes inherited)
    method_names: HashMap<String, String>,
    /// parent class name (if any)
    parent_name: Option<String>,
    total_fields: u32,                      // including type-id at slot 0
    /// Attribute name -> declared type name (e.g. "String", "Number", "Boolean")
    attr_types: HashMap<String, String>,
    /// method_name -> return type hint ("Number", "String", "Boolean", or class name)
    method_ret_types: HashMap<String, String>,
}

struct Ctx<'a> {
    preamble: String,
    globals: String,
    functions: String,
    counter: usize,
    scopes: Vec<HashMap<String, (String, ValTy)>>, // name -> (ptr, type)
    classes: HashMap<String, ClassLayout>,
    current_class: Option<String>,
    /// Tracks how many GC slots have been registered (global alloc list size)
    gc_alloc_count: usize,
    #[allow(dead_code)]
    sem: &'a Context,
    /// next type-id for class registration
    next_type_id: u32,
    /// Deferred lambda function definitions (emitted outside current function)
    lambda_defs: String,
    /// Tracks return types of named functions
    func_ret_types: HashMap<String, ValTy>,
}

/// Lightweight tag so we know how to load / print a value.
#[derive(Clone, Debug, PartialEq)]
#[allow(dead_code)]
enum ValTy { Num, Bool, Str, Ptr, Obj(String), Fn(Box<ValTy>) }

impl<'a> Ctx<'a> {
    fn new(sem: &'a Context) -> Self {
        let preamble = "\
; HULK -> LLVM IR  (generated)\n\
declare i32 @printf(i8*, ...)\n\
declare i32 @puts(i8*)\n\
declare i8* @malloc(i64)\n\
declare i8* @realloc(i8*, i64)\n\
declare void @free(i8*)\n\
declare i64 @strlen(i8*)\n\
declare i8* @strcpy(i8*, i8*)\n\
declare i8* @strcat(i8*, i8*)\n\
declare i32 @snprintf(i8*, i64, i8*, ...)\n\
declare i32 @rand()\n\
declare void @srand(i32)\n\
declare i64 @time(i64*)\n\
declare void @abort()\n\
declare double @llvm.pow.f64(double, double)\n\
declare double @llvm.sin.f64(double)\n\
declare double @llvm.cos.f64(double)\n\
declare double @llvm.exp.f64(double)\n\
declare double @llvm.log.f64(double)\n\
declare double @llvm.sqrt.f64(double)\n\
declare double @llvm.fabs.f64(double)\n\
\n\
@.fmt_num  = private unnamed_addr constant [5 x i8] c\"%.6g\\00\"\n\
@.fmt_str  = private unnamed_addr constant [3 x i8] c\"%s\\00\"\n\
@.fmt_nl   = private unnamed_addr constant [2 x i8] c\"\\0A\\00\"\n\
@.true_s   = private unnamed_addr constant [5 x i8] c\"true\\00\"\n\
@.false_s  = private unnamed_addr constant [6 x i8] c\"false\\00\"\n\
@.space_s  = private unnamed_addr constant [2 x i8] c\" \\00\"\n\
@.empty_s  = private unnamed_addr constant [1 x i8] c\"\\00\"\n\
@.oob_msg  = private unnamed_addr constant [36 x i8] c\"Runtime error: index out of bounds\\0A\\00\"\n\
@.rand_seeded = global i1 false\n\
\n\
; GC tracking: a growable array of i8* pointers\n\
@.gc_buf   = global i8** null\n\
@.gc_len   = global i64 0\n\
@.gc_cap   = global i64 0\n\
".to_string();

        Ctx {
            preamble,
            globals: String::new(),
            functions: String::new(),
            counter: 0,
            scopes: vec![HashMap::new()],
            classes: HashMap::new(),
            current_class: None,
            gc_alloc_count: 0,
            sem,
            next_type_id: 1, // 0 = reserved / unknown
            lambda_defs: String::new(),
            func_ret_types: HashMap::new(),
        }
    }

    // ── helpers ──────────────────────────────────────────────────────────────
    fn tmp(&mut self) -> String { let n = format!("%t{}", self.counter); self.counter += 1; n }
    fn lbl(&mut self, pfx: &str) -> String { let n = format!("{}_{}", pfx, self.counter); self.counter += 1; n }
    fn emit(&mut self, s: &str) { self.functions.push_str("  "); self.functions.push_str(s); self.functions.push('\n'); }
    fn emit_label(&mut self, l: &str) { self.functions.push_str(l); self.functions.push_str(":\n"); }

    fn add_global_string(&mut self, value: &str) -> String {
        let id = format!("@.slit_{}", self.counter);
        self.counter += 1;
        let len = value.len() + 1;
        self.globals.push_str(&format!(
            "{} = private unnamed_addr constant [{} x i8] c\"{}\\00\"\n", id, len, escape_llvm(value)
        ));
        id
    }

    fn gep_string(&self, id: &str, len: usize) -> String {
        format!("getelementptr inbounds ([{} x i8], [{} x i8]* {}, i64 0, i64 0)", len, len, id)
    }

    fn enter_scope(&mut self) { self.scopes.push(HashMap::new()); }
    fn exit_scope(&mut self)  { self.scopes.pop(); }

    fn def_var(&mut self, name: &str, ptr: &str, ty: ValTy) {
        if let Some(s) = self.scopes.last_mut() {
            s.insert(name.to_string(), (ptr.to_string(), ty));
        }
    }
    fn get_var(&self, name: &str) -> Option<(String, ValTy)> {
        for s in self.scopes.iter().rev() {
            if let Some(v) = s.get(name) { return Some(v.clone()); }
        }
        None
    }

    /// Safely convert a `double` value (pointer-as-double encoding) to a typed pointer.
    /// This spills the i64 through a stack alloca to force LLVM to use a GPR,
    /// avoiding a codegen issue where `bitcast double → i64` + `inttoptr` can
    /// leave the value stranded in an XMM register.
    /// Returns the name of the pointer SSA value of the given `target_ty` (e.g. "i8*", "double*", "i64*").
    fn decode_ptr(&mut self, double_val: &str, target_ty: &str) -> String {
        let i = self.tmp(); self.emit(&format!("{} = bitcast double {} to i64", i, double_val));
        let a = self.tmp(); self.emit(&format!("{} = alloca i64", a));
        self.emit(&format!("store i64 {}, i64* {}", i, a));
        let i2 = self.tmp(); self.emit(&format!("{} = load i64, i64* {}", i2, a));
        let p = self.tmp(); self.emit(&format!("{} = inttoptr i64 {} to {}", p, i2, target_ty));
        p
    }

    /// Emit a call to register a malloc'd pointer with the GC tracker.
    fn gc_track(&mut self, ptr: &str) {
        self.gc_alloc_count += 1;
        self.emit(&format!("call void @__hulk_gc_track(i8* {})", ptr));
    }

    /// Allocate a type-id for a class.
    fn alloc_type_id(&mut self) -> u32 {
        let id = self.next_type_id;
        self.next_type_id += 1;
        id
    }
}

fn escape_llvm(s: &str) -> String {
    s.replace('\\', "\\5C")
     .replace('\n', "\\0A")
     .replace('\t', "\\09")
     .replace('\"', "\\22")
}

/// Format a f64 as an LLVM IR double literal.
/// LLVM requires a decimal point in float constants (e.g. `1.0e1` not `1e1`).
fn fmt_double(v: f64) -> String {
    let s = format!("{:e}", v);
    // If the mantissa part has no '.', insert ".0" before the 'e'
    if let Some(pos) = s.find('e') {
        let mantissa = &s[..pos];
        if !mantissa.contains('.') {
            return format!("{}.0e{}", mantissa, &s[pos+1..]);
        }
    }
    // fallback: if no 'e', ensure a dot
    if !s.contains('.') && !s.contains('e') {
        return format!("{}.0", s);
    }
    s
}

// ─────────────────────────────────────────────────────────────────────────────
// GC helper functions (emitted as LLVM IR)
// ─────────────────────────────────────────────────────────────────────────────

/// Emit @__hulk_gc_track – appends a pointer to the growable GC buffer.
/// Emit @__hulk_gc_sweep – frees all tracked pointers at program exit.
fn emit_gc_sweep_fn(ctx: &mut Ctx) {
    // __hulk_gc_track(i8* ptr)
    ctx.functions.push_str("\
define void @__hulk_gc_track(i8* %ptr) {
entry:
  %len = load i64, i64* @.gc_len
  %cap = load i64, i64* @.gc_cap
  %need_grow = icmp sge i64 %len, %cap
  br i1 %need_grow, label %grow, label %store

grow:
  %new_cap_base = mul i64 %cap, 2
  %new_cap_min  = add i64 %new_cap_base, 16
  %new_cap = select i1 %need_grow, i64 %new_cap_min, i64 %new_cap_base
  %byte_sz = mul i64 %new_cap, 8
  %old_buf = load i8**, i8*** @.gc_buf
  %old_raw = bitcast i8** %old_buf to i8*
  %new_raw = call i8* @realloc(i8* %old_raw, i64 %byte_sz)
  %new_buf = bitcast i8* %new_raw to i8**
  store i8** %new_buf, i8*** @.gc_buf
  store i64 %new_cap, i64* @.gc_cap
  br label %store

store:
  %cur_buf = load i8**, i8*** @.gc_buf
  %cur_len = load i64, i64* @.gc_len
  %slot = getelementptr i8*, i8** %cur_buf, i64 %cur_len
  store i8* %ptr, i8** %slot
  %new_len = add i64 %cur_len, 1
  store i64 %new_len, i64* @.gc_len
  ret void
}

");

    // __hulk_gc_sweep()
    ctx.functions.push_str("\
define void @__hulk_gc_sweep() {
entry:
  %len = load i64, i64* @.gc_len
  %cmp0 = icmp sle i64 %len, 0
  br i1 %cmp0, label %done, label %loop_hdr

loop_hdr:
  %idx = alloca i64
  store i64 0, i64* %idx
  br label %loop

loop:
  %i = load i64, i64* %idx
  %cond = icmp slt i64 %i, %len
  br i1 %cond, label %body, label %free_buf

body:
  %buf = load i8**, i8*** @.gc_buf
  %slot = getelementptr i8*, i8** %buf, i64 %i
  %ptr = load i8*, i8** %slot
  call void @free(i8* %ptr)
  %next = add i64 %i, 1
  store i64 %next, i64* %idx
  br label %loop

free_buf:
  %buf2 = load i8**, i8*** @.gc_buf
  %buf_raw = bitcast i8** %buf2 to i8*
  call void @free(i8* %buf_raw)
  store i8** null, i8*** @.gc_buf
  store i64 0, i64* @.gc_len
  store i64 0, i64* @.gc_cap
  br label %done

done:
  ret void
}

");

    // ── @__hulk_num_to_str(double) -> i8*  ─  convert a number to a heap string
    ctx.functions.push_str("\
define i8* @__hulk_num_to_str(double %val) {
entry:
  ; First pass: measure needed length
  %len = call i32 (i8*, i64, i8*, ...) @snprintf(i8* null, i64 0, i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.fmt_num, i64 0, i64 0), double %val)
  %len64 = sext i32 %len to i64
  %bufsz = add i64 %len64, 1
  %buf = call i8* @malloc(i64 %bufsz)
  call void @__hulk_gc_track(i8* %buf)
  ; Second pass: actually format
  call i32 (i8*, i64, i8*, ...) @snprintf(i8* %buf, i64 %bufsz, i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.fmt_num, i64 0, i64 0), double %val)
  ret i8* %buf
}

");

    // ── @__hulk_bool_to_str(double) -> i8*  ─  returns pointer to \"true\" or \"false\"
    ctx.functions.push_str("\
define i8* @__hulk_bool_to_str(double %val) {
entry:
  %cond = fcmp one double %val, 0.0
  %res = select i1 %cond, i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.true_s, i64 0, i64 0), i8* getelementptr inbounds ([6 x i8], [6 x i8]* @.false_s, i64 0, i64 0)
  ret i8* %res
}

");

    // ── @__hulk_print_val(double)  ─  runtime print dispatcher
    //    Uses a tag passed as the second argument:
    //      0 = number, 1 = string (pointer), 2 = boolean
    //    Fallback: print as number (safe default).
    ctx.functions.push_str("\
define void @__hulk_print_val(double %val) {
entry:
  ; Fallback: print as number (safe default for Unknown type hint)
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.fmt_num, i64 0, i64 0), double %val)
  call i32 @puts(i8* getelementptr inbounds ([1 x i8], [1 x i8]* @.empty_s, i64 0, i64 0))
  ret void
}

");

    // ── @__hulk_to_str(double) -> i8*  ─  runtime convert to string
    //    Safe fallback: always treat as number. Strings should never reach here
    //    because expr_type_hint should detect them at compile time.
    ctx.functions.push_str("\
define i8* @__hulk_to_str(double %val) {
entry:
  %numstr = call i8* @__hulk_num_to_str(double %val)
  ret i8* %numstr
}

");
}

// ─────────────────────────────────────────────────────────────────────────────
// Class / Type emission  (with deep-inheritance support)
// ─────────────────────────────────────────────────────────────────────────────
fn emit_class(ctx: &mut Ctx, td: &TypeDecl) {
    // ── 1. Collect inherited attrs + methods from parent ────────────────
    let mut attr_indices: HashMap<String, u32> = HashMap::new();
    let mut attr_order: Vec<String> = Vec::new();
    let mut method_names: HashMap<String, String> = HashMap::new();
    let mut attr_types: HashMap<String, String> = HashMap::new();
    let mut parent_name: Option<String> = None;
    let mut idx: u32 = 1; // slot 0 = type-id (i32, padded to 8 bytes via i64 store)

    if let Some(ref pi) = td.parent {
        parent_name = Some(pi.name.clone());
        // Copy everything from parent layout
        if let Some(parent_layout) = ctx.classes.get(&pi.name) {
            for attr_name in &parent_layout.attr_order {
                let parent_idx = parent_layout.attr_indices[attr_name];
                attr_indices.insert(attr_name.clone(), parent_idx);
                attr_order.push(attr_name.clone());
                if parent_idx >= idx { idx = parent_idx + 1; }
            }
            // Inherit methods (child can override later)
            for (mname, mfn) in &parent_layout.method_names {
                method_names.insert(mname.clone(), mfn.clone());
            }
            // Inherit attribute types
            for (aname, atype) in &parent_layout.attr_types {
                attr_types.insert(aname.clone(), atype.clone());
            }
        }
    }

    // ── 2. Own attributes ────────────────────────────────────────────────
    // Build param type map: param_name -> type_name  (e.g., "name" -> "String")
    let param_types: HashMap<String, String> = td.params.iter().filter_map(|p| {
        if let Some(TypeAnnotation::Name(ref tn)) = p.type_annotation {
            Some((p.name.clone(), tn.clone()))
        } else {
            None
        }
    }).collect();

    for attr in &td.attributes {
        if !attr_indices.contains_key(&attr.name) {
            attr_indices.insert(attr.name.clone(), idx);
            attr_order.push(attr.name.clone());
            idx += 1;
        }
        // Infer attribute type from:
        //  a) Explicit attribute type annotation
        //  b) Init expression referencing a typed constructor param
        //  c) Init expression literal type
        if !attr_types.contains_key(&attr.name) {
            if let Some(TypeAnnotation::Name(ref tn)) = attr.type_annotation {
                attr_types.insert(attr.name.clone(), tn.clone());
            } else if let Expr::Identifier(ref pname) = attr.init.node {
                if let Some(tn) = param_types.get(pname) {
                    attr_types.insert(attr.name.clone(), tn.clone());
                }
            } else if matches!(attr.init.node, Expr::String(_)) {
                attr_types.insert(attr.name.clone(), "String".to_string());
            } else if matches!(attr.init.node, Expr::Number(_)) {
                attr_types.insert(attr.name.clone(), "Number".to_string());
            } else if matches!(attr.init.node, Expr::Boolean(_)) {
                attr_types.insert(attr.name.clone(), "Boolean".to_string());
            }
        }
    }

    // ── 3. Struct type { i64 typeid, double, double, ... } ──────────────
    let struct_name = format!("%T.{}", td.name);
    let mut fields = "i64".to_string(); // slot 0 = type id
    for _ in 1..idx { fields.push_str(", double"); }

    ctx.globals.push_str(&format!("{} = type {{ {} }}\n", struct_name, fields));

    // ── 4. Register own methods (override inherited) ────────────────────
    let mut method_ret_types: HashMap<String, String> = HashMap::new();
    // Inherit method return types from parent
    if let Some(ref pi) = td.parent {
        if let Some(parent_layout) = ctx.classes.get(&pi.name) {
            for (mname, mret) in &parent_layout.method_ret_types {
                method_ret_types.insert(mname.clone(), mret.clone());
            }
        }
    }
    for m in &td.methods {
        method_names.insert(m.name.clone(), format!("@{}_{}", td.name, m.name));
        // Infer method return type from explicit annotation or body heuristic
        if let Some(TypeAnnotation::Name(ref tn)) = m.return_type {
            method_ret_types.insert(m.name.clone(), tn.clone());
        } else {
            // Try to infer from the method body expression
            if let Some(rt) = infer_return_type_from_body(&m.body.node, &attr_types) {
                method_ret_types.insert(m.name.clone(), rt);
            }
        }
    }

    let type_id = ctx.alloc_type_id();

    ctx.classes.insert(td.name.clone(), ClassLayout {
        struct_name: struct_name.clone(),
        type_id,
        attr_indices: attr_indices.clone(),
        attr_order: attr_order.clone(),
        method_names: method_names.clone(),
        parent_name: parent_name.clone(),
        total_fields: idx,
        attr_types: attr_types.clone(),
        method_ret_types,
    });

    // ── 5. Emit constructor: @Type_new(args...) -> i8* ──────────────────
    {
        let mut sig = String::new();
        for (i, p) in td.params.iter().enumerate() {
            if i > 0 { sig.push_str(", "); }
            sig.push_str(&format!("double %{}", p.name));
        }
        ctx.functions.push_str(&format!("define i8* @{}_new({}) {{\nentry:\n", td.name, sig));

        let sz = idx as u64 * 8;
        let raw = ctx.tmp();
        ctx.emit(&format!("{} = call i8* @malloc(i64 {})", raw, sz));
        // GC track
        ctx.emit(&format!("call void @__hulk_gc_track(i8* {})", raw));

        let typed = ctx.tmp();
        ctx.emit(&format!("{} = bitcast i8* {} to {}*", typed, raw, struct_name));

        // Store type-id at slot 0
        let gep_tid = ctx.tmp();
        ctx.emit(&format!("{} = getelementptr inbounds {}, {}* {}, i32 0, i32 0",
            gep_tid, struct_name, struct_name, typed));
        ctx.emit(&format!("store i64 {}, i64* {}", type_id, gep_tid));

        // Constructor params in scope
        ctx.enter_scope();
        for p in &td.params {
            let ptr = ctx.tmp();
            ctx.emit(&format!("{} = alloca double", ptr));
            ctx.emit(&format!("store double %{}, double* {}", p.name, ptr));
            ctx.def_var(&p.name, &ptr, val_ty_from_annotation(&p.type_annotation));
        }

        // If parent has a constructor and parent init args are given, call parent
        // constructor to initialize inherited attrs.
        if let Some(ref pi) = td.parent {
            if ctx.classes.contains_key(&pi.name) {
                // Evaluate parent constructor args
                let mut parent_vals = Vec::new();
                for a in &pi.args {
                    parent_vals.push(gen_expr(ctx, a));
                }
                // Call parent constructor to get a temp parent obj, then copy its attrs
                let mut parent_arg_s = String::new();
                for (i, v) in parent_vals.iter().enumerate() {
                    if i > 0 { parent_arg_s.push_str(", "); }
                    parent_arg_s.push_str("double "); parent_arg_s.push_str(v);
                }
                let parent_raw = ctx.tmp();
                ctx.emit(&format!("{} = call i8* @{}_new({})", parent_raw, pi.name, parent_arg_s));

                // Copy parent attrs from parent_raw into our object
                let parent_sn = ctx.classes.get(&pi.name).map(|l| l.struct_name.clone())
                    .unwrap_or_default();
                let parent_typed = ctx.tmp();
                ctx.emit(&format!("{} = bitcast i8* {} to {}*", parent_typed, parent_raw, parent_sn));

                if let Some(parent_layout) = ctx.classes.get(&pi.name) {
                    let parent_attr_order = parent_layout.attr_order.clone();
                    let parent_attr_indices = parent_layout.attr_indices.clone();
                    let parent_sn2 = parent_layout.struct_name.clone();
                    for attr_name in &parent_attr_order {
                        let pidx = parent_attr_indices[attr_name];
                        let gep_src = ctx.tmp();
                        ctx.emit(&format!("{} = getelementptr inbounds {}, {}* {}, i32 0, i32 {}",
                            gep_src, parent_sn2, parent_sn2, parent_typed, pidx));
                        let val = ctx.tmp();
                        ctx.emit(&format!("{} = load double, double* {}", val, gep_src));
                        // Store into child at same index
                        let child_idx = attr_indices[attr_name];
                        let gep_dst = ctx.tmp();
                        ctx.emit(&format!("{} = getelementptr inbounds {}, {}* {}, i32 0, i32 {}",
                            gep_dst, struct_name, struct_name, typed, child_idx));
                        ctx.emit(&format!("store double {}, double* {}", val, gep_dst));
                    }
                }
            }
        }

        // Init own attributes
        for attr in &td.attributes {
            let val = gen_expr(ctx, &attr.init);
            let i = attr_indices[&attr.name];
            let gep = ctx.tmp();
            ctx.emit(&format!("{} = getelementptr inbounds {}, {}* {}, i32 0, i32 {}",
                gep, struct_name, struct_name, typed, i));
            ctx.emit(&format!("store double {}, double* {}", val, gep));
        }

        ctx.exit_scope();
        ctx.emit(&format!("ret i8* {}", raw));
        ctx.functions.push_str("}\n\n");
    }

    // ── 6. Emit methods ─────────────────────────────────────────────────
    ctx.current_class = Some(td.name.clone());
    for m in &td.methods {
        let fname = format!("{}_{}", td.name, m.name);
        let mut sig = String::from("i8* %self");
        for p in &m.params {
            sig.push_str(&format!(", double %{}", p.name));
        }
        ctx.functions.push_str(&format!("define double @{}({}) {{\nentry:\n", fname, sig));
        ctx.enter_scope();

        for p in &m.params {
            let ptr = ctx.tmp();
            ctx.emit(&format!("{} = alloca double", ptr));
            ctx.emit(&format!("store double %{}, double* {}", p.name, ptr));
            ctx.def_var(&p.name, &ptr, val_ty_from_annotation(&p.type_annotation));
        }

        let res = gen_expr(ctx, &m.body);
        ctx.emit(&format!("ret double {}", res));
        ctx.functions.push_str("}\n\n");
        ctx.exit_scope();
    }
    ctx.current_class = None;
}

// ─────────────────────────────────────────────────────────────────────────────
// Top-level function emission
// ─────────────────────────────────────────────────────────────────────────────
/// Mangle a HULK function name so it doesn't collide with the C entry @main.
fn mangle_fn(name: &str) -> String {
    if name == "main" { "__hulk_main".to_string() } else { name.to_string() }
}

fn emit_function(ctx: &mut Ctx, fd: &FunctionDecl) {
    // Register function return type
    let ret_vty = match &fd.return_type {
        Some(TypeAnnotation::Name(n)) => match n.as_str() {
            "String" => ValTy::Str,
            "Number" => ValTy::Num,
            "Boolean" => ValTy::Bool,
            other => ValTy::Obj(other.to_string()),
        },
        _ => ValTy::Num,
    };
    ctx.func_ret_types.insert(fd.name.clone(), ret_vty);

    let ir_name = mangle_fn(&fd.name);
    let mut sig = String::new();
    for (i, p) in fd.params.iter().enumerate() {
        if i > 0 { sig.push_str(", "); }
        sig.push_str(&format!("double %{}", p.name));
    }
    ctx.functions.push_str(&format!("define double @{}({}) {{\nentry:\n", ir_name, sig));
    ctx.enter_scope();

    for p in &fd.params {
        let ptr = ctx.tmp();
        ctx.emit(&format!("{} = alloca double", ptr));
        ctx.emit(&format!("store double %{}, double* {}", p.name, ptr));
        ctx.def_var(&p.name, &ptr, val_ty_from_annotation(&p.type_annotation));
    }

    let res = gen_expr(ctx, &fd.body);
    ctx.emit(&format!("ret double {}", res));
    ctx.functions.push_str("}\n\n");
    ctx.exit_scope();
}

// ─────────────────────────────────────────────────────────────────────────────
// Expression code generation  (covers every Expr variant)
// ─────────────────────────────────────────────────────────────────────────────
fn gen_expr(ctx: &mut Ctx, expr: &Spanned<Expr>) -> String {
    match &expr.node {
        // ── Primitives ──────────────────────────────────────────────────────
        Expr::Number(v) => fmt_double(*v),

        Expr::Boolean(v) => if *v { "1.0".into() } else { "0.0".into() },

        Expr::String(s) => {
            let id = ctx.add_global_string(s);
            let len = s.len() + 1;
            let gep = ctx.gep_string(&id, len);
            let i = ctx.tmp(); ctx.emit(&format!("{} = ptrtoint i8* {} to i64", i, gep));
            let d = ctx.tmp(); ctx.emit(&format!("{} = bitcast i64 {} to double", d, i));
            d
        }

        Expr::Identifier(name) => {
            // 1. Local / parameter
            if let Some((ptr, _ty)) = ctx.get_var(name) {
                let r = ctx.tmp();
                ctx.emit(&format!("{} = load double, double* {}", r, ptr));
                return r;
            }
            // 2. `self` reference inside a method — encode the i8* as double
            if name == "self" && ctx.current_class.is_some() {
                let i = ctx.tmp();
                ctx.emit(&format!("{} = ptrtoint i8* %self to i64", i));
                let d = ctx.tmp();
                ctx.emit(&format!("{} = bitcast i64 {} to double", d, i));
                return d;
            }
            // 3. self attribute (within a method body)
            if let Some(cls) = ctx.current_class.clone() {
                if let Some(layout) = ctx.classes.get(&cls) {
                    if let Some(&idx) = layout.attr_indices.get(name.as_str()) {
                        let sn = layout.struct_name.clone();
                        let cast = ctx.tmp();
                        ctx.emit(&format!("{} = bitcast i8* %self to {}*", cast, sn));
                        let gep = ctx.tmp();
                        ctx.emit(&format!("{} = getelementptr inbounds {}, {}* {}, i32 0, i32 {}",
                            gep, sn, sn, cast, idx));
                        let v = ctx.tmp();
                        ctx.emit(&format!("{} = load double, double* {}", v, gep));
                        return v;
                    }
                }
            }
            ctx.emit(&format!("; WARNING: variable '{}' not found", name));
            "0.0".into()
        }

        // ── Binary ──────────────────────────────────────────────────────────
        Expr::Binary(lhs_ast, op, rhs_ast) => {
            let l = gen_expr(ctx, lhs_ast);
            let r = gen_expr(ctx, rhs_ast);
            let res = ctx.tmp();

            match op {
                Op::Add => ctx.emit(&format!("{} = fadd double {}, {}", res, l, r)),
                Op::Sub => ctx.emit(&format!("{} = fsub double {}, {}", res, l, r)),
                Op::Mul => ctx.emit(&format!("{} = fmul double {}, {}", res, l, r)),
                Op::Div => ctx.emit(&format!("{} = fdiv double {}, {}", res, l, r)),
                Op::Mod => ctx.emit(&format!("{} = frem double {}, {}", res, l, r)),
                Op::Pow => ctx.emit(&format!("{} = call double @llvm.pow.f64(double {}, double {})", res, l, r)),

                Op::Eq | Op::Neq | Op::Lt | Op::Gt | Op::Le | Op::Ge => {
                    let pred = match op {
                        Op::Eq  => "oeq", Op::Neq => "one",
                        Op::Lt  => "olt", Op::Gt  => "ogt",
                        Op::Le  => "ole", Op::Ge  => "oge",
                        _ => unreachable!(),
                    };
                    let c = ctx.tmp();
                    ctx.emit(&format!("{} = fcmp {} double {}, {}", c, pred, l, r));
                    ctx.emit(&format!("{} = uitofp i1 {} to double", res, c));
                }
                Op::And => {
                    let lb = ctx.tmp(); ctx.emit(&format!("{} = fcmp one double {}, 0.0", lb, l));
                    let rb = ctx.tmp(); ctx.emit(&format!("{} = fcmp one double {}, 0.0", rb, r));
                    let ab = ctx.tmp(); ctx.emit(&format!("{} = and i1 {}, {}", ab, lb, rb));
                    ctx.emit(&format!("{} = uitofp i1 {} to double", res, ab));
                }
                Op::Or => {
                    let lb = ctx.tmp(); ctx.emit(&format!("{} = fcmp one double {}, 0.0", lb, l));
                    let rb = ctx.tmp(); ctx.emit(&format!("{} = fcmp one double {}, 0.0", rb, r));
                    let ob = ctx.tmp(); ctx.emit(&format!("{} = or i1 {}, {}", ob, lb, rb));
                    ctx.emit(&format!("{} = uitofp i1 {} to double", res, ob));
                }
                Op::Concat | Op::ConcatSpace => {
                    // Convert each operand to i8* string pointer.
                    // If the operand is already a string (pointer-as-double), bitcast.
                    // If it's a number, call @__hulk_num_to_str.
                    // If it's a bool, call @__hulk_bool_to_str.
                    // If unknown, use @__hulk_num_to_str as default heuristic.
                    let lp = gen_to_str_ptr(ctx, &lhs_ast.node, &l);
                    let rp = gen_to_str_ptr(ctx, &rhs_ast.node, &r);

                    let ll = ctx.tmp(); ctx.emit(&format!("{} = call i64 @strlen(i8* {})", ll, lp));
                    let rl = ctx.tmp(); ctx.emit(&format!("{} = call i64 @strlen(i8* {})", rl, rp));
                    let total = ctx.tmp(); ctx.emit(&format!("{} = add i64 {}, {}", total, ll, rl));
                    let extra = if *op == Op::ConcatSpace { 2i64 } else { 1 };
                    let total2 = ctx.tmp(); ctx.emit(&format!("{} = add i64 {}, {}", total2, total, extra));

                    let buf = ctx.tmp(); ctx.emit(&format!("{} = call i8* @malloc(i64 {})", buf, total2));
                    ctx.gc_track(&buf);
                    ctx.emit(&format!("call i8* @strcpy(i8* {}, i8* {})", buf, lp));
                    if *op == Op::ConcatSpace {
                        ctx.emit(&format!("call i8* @strcat(i8* {}, i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.space_s, i64 0, i64 0))", buf));
                    }
                    ctx.emit(&format!("call i8* @strcat(i8* {}, i8* {})", buf, rp));

                    let pi = ctx.tmp(); ctx.emit(&format!("{} = ptrtoint i8* {} to i64", pi, buf));
                    ctx.emit(&format!("{} = bitcast i64 {} to double", res, pi));
                }
            }
            res
        }

        // ── Unary ───────────────────────────────────────────────────────────
        Expr::Unary(op, operand) => {
            let v = gen_expr(ctx, operand);
            let res = ctx.tmp();
            match op {
                UnOp::Neg => ctx.emit(&format!("{} = fneg double {}", res, v)),
                UnOp::Not => {
                    let c = ctx.tmp();
                    ctx.emit(&format!("{} = fcmp oeq double {}, 0.0", c, v));
                    ctx.emit(&format!("{} = uitofp i1 {} to double", res, c));
                }
            }
            res
        }

        // ── If / Else ───────────────────────────────────────────────────────
        Expr::If { cond, then_expr, else_expr } => {
            let cv = gen_expr(ctx, cond);
            let cb = ctx.tmp();
            ctx.emit(&format!("{} = fcmp one double {}, 0.0", cb, cv));

            let res_ptr = ctx.tmp();
            ctx.emit(&format!("{} = alloca double", res_ptr));

            let tl = ctx.lbl("then"); let el = ctx.lbl("else"); let ml = ctx.lbl("merge");
            ctx.emit(&format!("br i1 {}, label %{}, label %{}", cb, tl, el));

            ctx.emit_label(&tl);
            let tv = gen_expr(ctx, then_expr);
            ctx.emit(&format!("store double {}, double* {}", tv, res_ptr));
            ctx.emit(&format!("br label %{}", ml));

            ctx.emit_label(&el);
            let ev = gen_expr(ctx, else_expr);
            ctx.emit(&format!("store double {}, double* {}", ev, res_ptr));
            ctx.emit(&format!("br label %{}", ml));

            ctx.emit_label(&ml);
            let r = ctx.tmp();
            ctx.emit(&format!("{} = load double, double* {}", r, res_ptr));
            r
        }

        // ── While ───────────────────────────────────────────────────────────
        Expr::While { cond, body } => {
            let lc = ctx.lbl("wcond"); let lb = ctx.lbl("wbody"); let le = ctx.lbl("wend");
            ctx.emit(&format!("br label %{}", lc));

            ctx.emit_label(&lc);
            let cv = gen_expr(ctx, cond);
            let cb = ctx.tmp();
            ctx.emit(&format!("{} = fcmp one double {}, 0.0", cb, cv));
            ctx.emit(&format!("br i1 {}, label %{}, label %{}", cb, lb, le));

            ctx.emit_label(&lb);
            gen_expr(ctx, body);
            ctx.emit(&format!("br label %{}", lc));

            ctx.emit_label(&le);
            "0.0".into()
        }

        // ── For ─────────────────────────────────────────────────────────────
        Expr::For { var, iterable, body } => {
            let iter_val = gen_expr(ctx, iterable);
            let vp = ctx.decode_ptr(&iter_val, "double*");

            let len_ptr = vp.clone();
            let len_d = ctx.tmp(); ctx.emit(&format!("{} = load double, double* {}", len_d, len_ptr));
            let len_i = ctx.tmp(); ctx.emit(&format!("{} = fptosi double {} to i64", len_i, len_d));

            let idx_ptr = ctx.tmp(); ctx.emit(&format!("{} = alloca i64", idx_ptr));
            ctx.emit(&format!("store i64 0, i64* {}", idx_ptr));

            let lc = ctx.lbl("fcond"); let lb = ctx.lbl("fbody"); let le = ctx.lbl("fend");
            ctx.emit(&format!("br label %{}", lc));

            ctx.emit_label(&lc);
            let ci = ctx.tmp(); ctx.emit(&format!("{} = load i64, i64* {}", ci, idx_ptr));
            let cc = ctx.tmp(); ctx.emit(&format!("{} = icmp slt i64 {}, {}", cc, ci, len_i));
            ctx.emit(&format!("br i1 {}, label %{}, label %{}", cc, lb, le));

            ctx.emit_label(&lb);
            ctx.enter_scope();
            let ci2 = ctx.tmp(); ctx.emit(&format!("{} = load i64, i64* {}", ci2, idx_ptr));
            let off = ctx.tmp(); ctx.emit(&format!("{} = add i64 {}, 1", off, ci2));
            let ep = ctx.tmp(); ctx.emit(&format!("{} = getelementptr double, double* {}, i64 {}", ep, vp, off));
            let ev = ctx.tmp(); ctx.emit(&format!("{} = load double, double* {}", ev, ep));
            let vp2 = ctx.tmp(); ctx.emit(&format!("{} = alloca double", vp2));
            ctx.emit(&format!("store double {}, double* {}", ev, vp2));
            ctx.def_var(var, &vp2, ValTy::Num);

            gen_expr(ctx, body);

            let ni = ctx.tmp(); ctx.emit(&format!("{} = load i64, i64* {}", ni, idx_ptr));
            let ni2 = ctx.tmp(); ctx.emit(&format!("{} = add i64 {}, 1", ni2, ni));
            ctx.emit(&format!("store i64 {}, i64* {}", ni2, idx_ptr));
            ctx.exit_scope();
            ctx.emit(&format!("br label %{}", lc));

            ctx.emit_label(&le);
            "0.0".into()
        }

        // ── Let ─────────────────────────────────────────────────────────────
        Expr::Let { bindings, body } => {
            ctx.enter_scope();
            for (name, _ann, init_expr) in bindings {
                let vty = infer_val_ty(ctx, &init_expr.node);
                let v = gen_expr(ctx, init_expr);
                let p = ctx.tmp();
                ctx.emit(&format!("{} = alloca double", p));
                ctx.emit(&format!("store double {}, double* {}", v, p));
                ctx.def_var(name, &p, vty);
            }
            let res = gen_expr(ctx, body);
            ctx.exit_scope();
            res
        }

        // ── Assignment ──────────────────────────────────────────────────────
        Expr::Assignment { target, value } => {
            let v = gen_expr(ctx, value);
            if let Some((ptr, _)) = ctx.get_var(target) {
                ctx.emit(&format!("store double {}, double* {}", v, ptr));
            }
            v
        }

        // ── Block ───────────────────────────────────────────────────────────
        Expr::Block(exprs) => {
            let mut last = "0.0".to_string();
            for e in exprs { last = gen_expr(ctx, e); }
            last
        }

        // ── Call (function) ─────────────────────────────────────────────────
        Expr::Call { func, args } => {
            if func == "print" {
                gen_print(ctx, args);
                return "0.0".into();
            }
            let mut vals = Vec::new();
            for a in args { vals.push(gen_expr(ctx, a)); }

            // Check if 'func' is a variable (lambda/closure) in scope
            if let Some((ptr, vty)) = ctx.get_var(func) {
                // ── Functor call: if the variable holds an object with an `invoke` method,
                // call obj.invoke(args) instead of treating it as a closure.
                if let ValTy::Obj(ref class_name) = vty {
                    if let Some(layout) = ctx.classes.get(class_name.as_str()) {
                        if let Some(invoke_fname) = layout.method_names.get("invoke") {
                            let invoke_fname = invoke_fname[1..].to_string(); // strip leading @
                            let ov = ctx.tmp();
                            ctx.emit(&format!("{} = load double, double* {}", ov, ptr));
                            let op = ctx.decode_ptr(&ov, "i8*");
                            let mut arg_s = format!("i8* {}", op);
                            for v in &vals {
                                arg_s.push_str(&format!(", double {}", v));
                            }
                            let r = ctx.tmp();
                            ctx.emit(&format!("{} = call double @{}({})", r, invoke_fname, arg_s));
                            return r;
                        }
                    }
                }

                // Closure call: variable holds a double-encoded pointer to [fn_ptr, env_ptr]
                let cval = ctx.tmp();
                ctx.emit(&format!("{} = load double, double* {}", cval, ptr));
                let cp = ctx.decode_ptr(&cval, "double*");
                // Load fn_ptr (slot 0)
                let fp_d = ctx.tmp();
                ctx.emit(&format!("{} = load double, double* {}", fp_d, cp));
                // Build fn type: double(double*, double, double, ...)
                let param_types: String = std::iter::once("double*".to_string())
                    .chain(vals.iter().map(|_| "double".to_string()))
                    .collect::<Vec<_>>().join(", ");
                let fn_ty = format!("double ({})*", param_types);
                let fp = ctx.decode_ptr(&fp_d, &fn_ty);
                // Load env_ptr (slot 1)
                let env_slot = ctx.tmp();
                ctx.emit(&format!("{} = getelementptr double, double* {}, i64 1", env_slot, cp));
                let env_d = ctx.tmp();
                ctx.emit(&format!("{} = load double, double* {}", env_d, env_slot));
                let env_p = ctx.decode_ptr(&env_d, "double*");
                // Build args: env_ptr, val0, val1, ...
                let mut arg_s = format!("double* {}", env_p);
                for v in &vals {
                    arg_s.push_str(&format!(", double {}", v));
                }
                let r = ctx.tmp();
                ctx.emit(&format!("{} = call double {}({})", r, fp, arg_s));
                r
            } else {
                // Direct function call
                let mut arg_s = String::new();
                for (i, v) in vals.iter().enumerate() {
                    if i > 0 { arg_s.push_str(", "); }
                    arg_s.push_str("double "); arg_s.push_str(v);
                }
                let r = ctx.tmp();
                let ir_func = mangle_fn(func);
                ctx.emit(&format!("{} = call double @{}({})", r, ir_func, arg_s));
                r
            }
        }

        // ── Instantiation ───────────────────────────────────────────────────
        Expr::Instantiation { ty, args } => {
            let mut vals = Vec::new();
            for a in args { vals.push(gen_expr(ctx, a)); }
            let mut arg_s = String::new();
            for (i, v) in vals.iter().enumerate() {
                if i > 0 { arg_s.push_str(", "); }
                arg_s.push_str("double "); arg_s.push_str(v);
            }
            let raw = ctx.tmp();
            ctx.emit(&format!("{} = call i8* @{}_new({})", raw, ty, arg_s));
            let pi = ctx.tmp(); ctx.emit(&format!("{} = ptrtoint i8* {} to i64", pi, raw));
            let d = ctx.tmp(); ctx.emit(&format!("{} = bitcast i64 {} to double", d, pi));
            d
        }

        // ── MethodCall ──────────────────────────────────────────────────────
        Expr::MethodCall { obj, method, args } => {
            // Resolve the class of the object at compile time
            let obj_class = resolve_obj_class_from_expr(ctx, &obj.node);
            let ov = gen_expr(ctx, obj);
            let op = ctx.decode_ptr(&ov, "i8*");

            let mut vals = Vec::new();
            for a in args { vals.push(gen_expr(ctx, a)); }

            let mut func_name = format!("unknown_{}", method);
            // First try the resolved class
            if let Some(ref cls) = obj_class {
                if let Some(layout) = ctx.classes.get(cls.as_str()) {
                    if let Some(fname) = layout.method_names.get(method.as_str()) {
                        func_name = fname[1..].to_string();
                    }
                }
            }
            // Fallback: search all classes
            if func_name.starts_with("unknown_") {
                for (_, layout) in &ctx.classes {
                    if let Some(fname) = layout.method_names.get(method.as_str()) {
                        func_name = fname[1..].to_string();
                        break;
                    }
                }
            }

            let mut arg_s = format!("i8* {}", op);
            for v in &vals {
                arg_s.push_str(&format!(", double {}", v));
            }
            let r = ctx.tmp();
            ctx.emit(&format!("{} = call double @{}({})", r, func_name, arg_s));
            r
        }

        // ── AttributeAccess ─────────────────────────────────────────────────
        Expr::AttributeAccess { obj, attribute } => {
            // Resolve object class at compile time for correct struct layout
            let obj_class = resolve_obj_class_from_expr(ctx, &obj.node);
            let ov = gen_expr(ctx, obj);
            let op = ctx.decode_ptr(&ov, "i8*");

            let mut found_idx: Option<(u32, String)> = None;
            // First try the resolved class
            if let Some(ref cls) = obj_class {
                if let Some(layout) = ctx.classes.get(cls.as_str()) {
                    if let Some(&idx) = layout.attr_indices.get(attribute.as_str()) {
                        found_idx = Some((idx, layout.struct_name.clone()));
                    }
                }
            }
            // Fallback: search all classes
            if found_idx.is_none() {
                for (_, layout) in &ctx.classes {
                    if let Some(&idx) = layout.attr_indices.get(attribute.as_str()) {
                        found_idx = Some((idx, layout.struct_name.clone()));
                        break;
                    }
                }
            }

            if let Some((idx, sn)) = found_idx {
                let cast = ctx.tmp(); ctx.emit(&format!("{} = bitcast i8* {} to {}*", cast, op, sn));
                let gep = ctx.tmp(); ctx.emit(&format!("{} = getelementptr inbounds {}, {}* {}, i32 0, i32 {}",
                    gep, sn, sn, cast, idx));
                let v = ctx.tmp(); ctx.emit(&format!("{} = load double, double* {}", v, gep));
                v
            } else {
                ctx.emit(&format!("; WARNING: attribute '{}' not found", attribute));
                "0.0".into()
            }
        }

        // ── BaseCall  (calls parent's constructor or method) ────────────────
        Expr::BaseCall { args } => {
            // Resolve the parent class from current_class
            let parent_info = ctx.current_class.as_ref().and_then(|cls| {
                ctx.classes.get(cls).and_then(|layout| {
                    layout.parent_name.as_ref().map(|pn| pn.clone())
                })
            });

            if let Some(parent_class) = parent_info {
                // Evaluate args
                let mut vals = Vec::new();
                for a in args { vals.push(gen_expr(ctx, a)); }
                let mut arg_s = String::new();
                for (i, v) in vals.iter().enumerate() {
                    if i > 0 { arg_s.push_str(", "); }
                    arg_s.push_str("double "); arg_s.push_str(v);
                }
                // Call parent constructor
                let raw = ctx.tmp();
                ctx.emit(&format!("{} = call i8* @{}_new({})", raw, parent_class, arg_s));
                let pi = ctx.tmp(); ctx.emit(&format!("{} = ptrtoint i8* {} to i64", pi, raw));
                let d = ctx.tmp(); ctx.emit(&format!("{} = bitcast i64 {} to double", d, pi));
                d
            } else {
                for a in args { gen_expr(ctx, a); }
                ctx.emit("; BaseCall – no parent class found");
                "0.0".into()
            }
        }

        // ── Lambda (with closure capture) ───────────────────────────────────
        Expr::Lambda { params, body, .. } => {
            // 1. Identify free variables in body that are not lambda params
            let param_names: Vec<String> = params.iter().map(|p| p.name.clone()).collect();
            let free_vars = collect_free_vars(body, &param_names);

            // 2. Capture current values of free variables
            let mut captured: Vec<(String, String)> = Vec::new(); // (name, current_value)
            for fv in &free_vars {
                if let Some((ptr, _)) = ctx.get_var(fv) {
                    let val = ctx.tmp();
                    ctx.emit(&format!("{} = load double, double* {}", val, ptr));
                    captured.push((fv.clone(), val));
                }
            }

            // 3. Allocate closure environment on heap: [cap0, cap1, ...]
            let env_size = captured.len();
            let env_ptr = if env_size > 0 {
                let bytes = env_size as u64 * 8;
                let raw = ctx.tmp();
                ctx.emit(&format!("{} = call i8* @malloc(i64 {})", raw, bytes));
                ctx.gc_track(&raw);
                let arr = ctx.tmp();
                ctx.emit(&format!("{} = bitcast i8* {} to double*", arr, raw));
                // Store captured values
                for (i, (_, val)) in captured.iter().enumerate() {
                    let gep = ctx.tmp();
                    ctx.emit(&format!("{} = getelementptr double, double* {}, i64 {}", gep, arr, i));
                    ctx.emit(&format!("store double {}, double* {}", val, gep));
                }
                Some(arr.clone())
            } else {
                None
            };

            // 4. Emit the lambda function body: @__lambda_N(double* %env, double %p0, ...)
            let fname = format!("__lambda_{}", ctx.counter); ctx.counter += 1;
            let mut sig = String::from("double* %__env");
            for p in params {
                sig.push_str(&format!(", double %{}", p.name));
            }

            // Save current functions buffer and start a fresh one for the lambda
            let saved_functions = std::mem::take(&mut ctx.functions);
            ctx.functions.push_str(&format!("define double @{}({}) {{\nentry:\n", fname, sig));
            ctx.enter_scope();

            // Load captured variables from env
            for (i, (name, _)) in captured.iter().enumerate() {
                let gep = ctx.tmp();
                ctx.emit(&format!("{} = getelementptr double, double* %__env, i64 {}", gep, i));
                let val = ctx.tmp();
                ctx.emit(&format!("{} = load double, double* {}", val, gep));
                let ptr = ctx.tmp();
                ctx.emit(&format!("{} = alloca double", ptr));
                ctx.emit(&format!("store double {}, double* {}", val, ptr));
                ctx.def_var(name, &ptr, ValTy::Num);
            }

            // Lambda params
            for p in params {
                let ptr = ctx.tmp();
                ctx.emit(&format!("{} = alloca double", ptr));
                ctx.emit(&format!("store double %{}, double* {}", p.name, ptr));
                ctx.def_var(&p.name, &ptr, val_ty_from_annotation(&p.type_annotation));
            }

            let r = gen_expr(ctx, body);
            ctx.emit(&format!("ret double {}", r));
            ctx.functions.push_str("}\n\n");
            ctx.exit_scope();

            // Move the lambda definition to lambda_defs and restore caller's buffer
            let lambda_code = std::mem::replace(&mut ctx.functions, saved_functions);
            ctx.lambda_defs.push_str(&lambda_code);

            // 5. Encode closure as a pair: { fn_ptr, env_ptr } packed into two doubles
            // For simplicity we pack [fn_ptr_as_double, env_ptr_as_double] in a heap buffer
            let closure_buf = ctx.tmp();
            ctx.emit(&format!("{} = call i8* @malloc(i64 16)", closure_buf)); // 2 * 8 bytes
            ctx.gc_track(&closure_buf);
            let closure_arr = ctx.tmp();
            ctx.emit(&format!("{} = bitcast i8* {} to double*", closure_arr, closure_buf));

            // Store fn ptr as double
            let fn_param_types: String = std::iter::once("double*".to_string())
                .chain(params.iter().map(|_| "double".to_string()))
                .collect::<Vec<_>>().join(", ");
            let fp_i = ctx.tmp();
            ctx.emit(&format!("{} = ptrtoint double ({})* @{} to i64", fp_i, fn_param_types, fname));
            let fp_d = ctx.tmp();
            ctx.emit(&format!("{} = bitcast i64 {} to double", fp_d, fp_i));
            ctx.emit(&format!("store double {}, double* {}", fp_d, closure_arr));

            // Store env ptr as double
            let env_val = if let Some(ref ep) = env_ptr {
                let ei = ctx.tmp();
                ctx.emit(&format!("{} = ptrtoint double* {} to i64", ei, ep));
                let ed = ctx.tmp();
                ctx.emit(&format!("{} = bitcast i64 {} to double", ed, ei));
                ed
            } else {
                "0.0".to_string()
            };
            let env_slot = ctx.tmp();
            ctx.emit(&format!("{} = getelementptr double, double* {}, i64 1", env_slot, closure_arr));
            ctx.emit(&format!("store double {}, double* {}", env_val, env_slot));

            // Return closure buffer as double
            let pi = ctx.tmp(); ctx.emit(&format!("{} = ptrtoint double* {} to i64", pi, closure_arr));
            let d = ctx.tmp(); ctx.emit(&format!("{} = bitcast i64 {} to double", d, pi));
            d
        }

        // ── Is (runtime type check) ─────────────────────────────────────────
        Expr::Is(expr, type_name) => {
            let v = gen_expr(ctx, expr);
            // Look up the type-id of the target type and all its descendants
            if let Some(target_layout) = ctx.classes.get(type_name.as_str()) {
                let target_tid = target_layout.type_id;
                // Collect target + all children type-ids
                let mut valid_ids = vec![target_tid];
                // Walk all classes and check if they inherit from type_name
                for (_, layout) in &ctx.classes {
                    if layout.type_id != target_tid && class_inherits_from(&ctx.classes, layout, type_name) {
                        valid_ids.push(layout.type_id);
                    }
                }

                // Decode object pointer and read type-id from slot 0
                let op = ctx.decode_ptr(&v, "i64*");
                let tid = ctx.tmp(); ctx.emit(&format!("{} = load i64, i64* {}", tid, op));

                // Check if tid matches any valid id
                let res_ptr = ctx.tmp();
                ctx.emit(&format!("{} = alloca double", res_ptr));
                ctx.emit(&format!("store double 0.0, double* {}", res_ptr));

                for vid in &valid_ids {
                    let c = ctx.tmp();
                    ctx.emit(&format!("{} = icmp eq i64 {}, {}", c, tid, vid));
                    let match_lbl = ctx.lbl("is_match");
                    let next_lbl = ctx.lbl("is_next");
                    ctx.emit(&format!("br i1 {}, label %{}, label %{}", c, match_lbl, next_lbl));
                    ctx.emit_label(&match_lbl);
                    ctx.emit(&format!("store double 1.0, double* {}", res_ptr));
                    // We can't easily break, but we overwrite with 1.0 which is fine
                    ctx.emit(&format!("br label %{}", next_lbl));
                    ctx.emit_label(&next_lbl);
                }

                let r = ctx.tmp();
                ctx.emit(&format!("{} = load double, double* {}", r, res_ptr));
                r
            } else {
                // Type not found in classes – check primitives
                // For Number/Boolean/String we can't check at runtime (everything is double)
                // Return 1.0 as best-effort
                gen_expr(ctx, expr);
                "1.0".into()
            }
        }

        // ── As (runtime type cast with check) ───────────────────────────────
        Expr::As(expr, type_name) => {
            let v = gen_expr(ctx, expr);
            // If the type has a type-id, verify and abort on failure
            if let Some(target_layout) = ctx.classes.get(type_name.as_str()) {
                let target_tid = target_layout.type_id;
                let mut valid_ids = vec![target_tid];
                for (_, layout) in &ctx.classes {
                    if layout.type_id != target_tid && class_inherits_from(&ctx.classes, layout, type_name) {
                        valid_ids.push(layout.type_id);
                    }
                }

                let op = ctx.decode_ptr(&v, "i64*");
                let tid = ctx.tmp(); ctx.emit(&format!("{} = load i64, i64* {}", tid, op));
                let mut any = None;
                for vid in &valid_ids {
                    let c = ctx.tmp();
                    ctx.emit(&format!("{} = icmp eq i64 {}, {}", c, tid, vid));
                    any = Some(if let Some(prev) = any {
                        let combined = ctx.tmp();
                        ctx.emit(&format!("{} = or i1 {}, {}", combined, prev, c));
                        combined
                    } else {
                        c
                    });
                }

                if let Some(ok_cond) = any {
                    let ok_lbl = ctx.lbl("as_ok");
                    let fail_lbl = ctx.lbl("as_fail");
                    ctx.emit(&format!("br i1 {}, label %{}, label %{}", ok_cond, ok_lbl, fail_lbl));
                    ctx.emit_label(&fail_lbl);
                    // Print error and abort
                    let msg_id = ctx.add_global_string("Runtime error: invalid cast");
                    let msg_len = "Runtime error: invalid cast".len() + 1;
                    let msg_gep = ctx.gep_string(&msg_id, msg_len);
                    ctx.emit(&format!("call i32 @puts(i8* {})", msg_gep));
                    ctx.emit("call void @abort()");
                    ctx.emit("unreachable");
                    ctx.emit_label(&ok_lbl);
                }
            }
            // Passthrough the value (it's still the same object)
            v
        }

        // ── VectorLiteral ───────────────────────────────────────────────────
        Expr::VectorLiteral(elems) => {
            let count = elems.len();
            let total = (count + 1) as u64 * 8;
            let raw = ctx.tmp();
            ctx.emit(&format!("{} = call i8* @malloc(i64 {})", raw, total));
            ctx.gc_track(&raw);
            let arr = ctx.tmp();
            ctx.emit(&format!("{} = bitcast i8* {} to double*", arr, raw));
            ctx.emit(&format!("store double {}, double* {}", fmt_double(count as f64), arr));
            for (i, e) in elems.iter().enumerate() {
                let v = gen_expr(ctx, e);
                let gep = ctx.tmp();
                ctx.emit(&format!("{} = getelementptr double, double* {}, i64 {}", gep, arr, i + 1));
                ctx.emit(&format!("store double {}, double* {}", v, gep));
            }
            let pi = ctx.tmp(); ctx.emit(&format!("{} = ptrtoint double* {} to i64", pi, arr));
            let d = ctx.tmp(); ctx.emit(&format!("{} = bitcast i64 {} to double", d, pi));
            d
        }

        // ── VectorGenerator [expr || var in iterable] ───────────────────────
        Expr::VectorGenerator { expr: elem_expr, var, iterable } => {
            let iter_val = gen_expr(ctx, iterable);

            let sp = ctx.decode_ptr(&iter_val, "double*");
            let len_d = ctx.tmp(); ctx.emit(&format!("{} = load double, double* {}", len_d, sp));
            let len_i = ctx.tmp(); ctx.emit(&format!("{} = fptosi double {} to i64", len_i, len_d));

            let one = ctx.tmp(); ctx.emit(&format!("{} = add i64 {}, 1", one, len_i));
            let bytes = ctx.tmp(); ctx.emit(&format!("{} = mul i64 {}, 8", bytes, one));
            let raw = ctx.tmp(); ctx.emit(&format!("{} = call i8* @malloc(i64 {})", raw, bytes));
            ctx.gc_track(&raw);
            let dp = ctx.tmp(); ctx.emit(&format!("{} = bitcast i8* {} to double*", dp, raw));
            ctx.emit(&format!("store double {}, double* {}", len_d, dp));

            let idx_ptr = ctx.tmp(); ctx.emit(&format!("{} = alloca i64", idx_ptr));
            ctx.emit(&format!("store i64 0, i64* {}", idx_ptr));

            let lc = ctx.lbl("vgc"); let lb = ctx.lbl("vgb"); let le = ctx.lbl("vge");
            ctx.emit(&format!("br label %{}", lc));
            ctx.emit_label(&lc);
            let ci = ctx.tmp(); ctx.emit(&format!("{} = load i64, i64* {}", ci, idx_ptr));
            let cc = ctx.tmp(); ctx.emit(&format!("{} = icmp slt i64 {}, {}", cc, ci, len_i));
            ctx.emit(&format!("br i1 {}, label %{}, label %{}", cc, lb, le));

            ctx.emit_label(&lb);
            ctx.enter_scope();
            let off = ctx.tmp(); ctx.emit(&format!("{} = add i64 {}, 1", off, ci));
            let ep = ctx.tmp(); ctx.emit(&format!("{} = getelementptr double, double* {}, i64 {}", ep, sp, off));
            let ev = ctx.tmp(); ctx.emit(&format!("{} = load double, double* {}", ev, ep));
            let vp = ctx.tmp(); ctx.emit(&format!("{} = alloca double", vp));
            ctx.emit(&format!("store double {}, double* {}", ev, vp));
            ctx.def_var(var, &vp, ValTy::Num);

            let mapped = gen_expr(ctx, elem_expr);
            let dp2 = ctx.tmp(); ctx.emit(&format!("{} = getelementptr double, double* {}, i64 {}", dp2, dp, off));
            ctx.emit(&format!("store double {}, double* {}", mapped, dp2));

            let ni = ctx.tmp(); ctx.emit(&format!("{} = add i64 {}, 1", ni, ci));
            ctx.emit(&format!("store i64 {}, i64* {}", ni, idx_ptr));
            ctx.exit_scope();
            ctx.emit(&format!("br label %{}", lc));

            ctx.emit_label(&le);
            let pi = ctx.tmp(); ctx.emit(&format!("{} = ptrtoint double* {} to i64", pi, dp));
            let d = ctx.tmp(); ctx.emit(&format!("{} = bitcast i64 {} to double", d, pi));
            d
        }

        // ── Indexing (with bounds checking) ─────────────────────────────────
        Expr::Indexing { obj, index } => {
            let ov = gen_expr(ctx, obj);
            let iv = gen_expr(ctx, index);
            let op = ctx.decode_ptr(&ov, "double*");
            let ii = ctx.tmp(); ctx.emit(&format!("{} = fptosi double {} to i64", ii, iv));

            // ── Bounds check: 0 <= ii < len ─────────────────────────────────
            let len_d = ctx.tmp(); ctx.emit(&format!("{} = load double, double* {}", len_d, op));
            let len_i = ctx.tmp(); ctx.emit(&format!("{} = fptosi double {} to i64", len_i, len_d));
            let neg_check = ctx.tmp(); ctx.emit(&format!("{} = icmp slt i64 {}, 0", neg_check, ii));
            let upper_check = ctx.tmp(); ctx.emit(&format!("{} = icmp sge i64 {}, {}", upper_check, ii, len_i));
            let oob = ctx.tmp(); ctx.emit(&format!("{} = or i1 {}, {}", oob, neg_check, upper_check));

            let ok_lbl = ctx.lbl("idx_ok");
            let fail_lbl = ctx.lbl("idx_oob");
            ctx.emit(&format!("br i1 {}, label %{}, label %{}", oob, fail_lbl, ok_lbl));

            ctx.emit_label(&fail_lbl);
            ctx.emit("call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([36 x i8], [36 x i8]* @.oob_msg, i64 0, i64 0))");
            ctx.emit("call void @abort()");
            ctx.emit("unreachable");

            ctx.emit_label(&ok_lbl);
            let off = ctx.tmp(); ctx.emit(&format!("{} = add i64 {}, 1", off, ii));
            let ep = ctx.tmp(); ctx.emit(&format!("{} = getelementptr double, double* {}, i64 {}", ep, op, off));
            let v = ctx.tmp(); ctx.emit(&format!("{} = load double, double* {}", v, ep));
            v
        }

        // ── Match ───────────────────────────────────────────────────────────
        Expr::Match { expr: match_expr, cases, default } => {
            let mv = gen_expr(ctx, match_expr);
            let res_ptr = ctx.tmp();
            ctx.emit(&format!("{} = alloca double", res_ptr));
            let end_lbl = ctx.lbl("mend");

            for case in cases {
                let next_lbl = ctx.lbl("mnext");
                match &case.pattern {
                    Pattern::Literal(lit) => {
                        let lv = match lit {
                            Expr::Number(n) => fmt_double(*n),
                            Expr::Boolean(b) => if *b { "1.0".into() } else { "0.0".into() },
                            _ => "0.0".into(),
                        };
                        let c = ctx.tmp();
                        ctx.emit(&format!("{} = fcmp oeq double {}, {}", c, mv, lv));
                        let body_lbl = ctx.lbl("mbody");
                        ctx.emit(&format!("br i1 {}, label %{}, label %{}", c, body_lbl, next_lbl));
                        ctx.emit_label(&body_lbl);
                    }
                    Pattern::Wildcard => { /* always match */ }
                    Pattern::Variable { name, .. } => {
                        ctx.enter_scope();
                        let p = ctx.tmp();
                        ctx.emit(&format!("{} = alloca double", p));
                        ctx.emit(&format!("store double {}, double* {}", mv, p));
                        ctx.def_var(name, &p, ValTy::Num);
                    }
                    _ => {
                        ctx.emit(&format!("br label %{}", next_lbl));
                    }
                }
                let cv = gen_expr(ctx, &case.expr);
                ctx.emit(&format!("store double {}, double* {}", cv, res_ptr));

                if matches!(&case.pattern, Pattern::Variable { .. }) {
                    ctx.exit_scope();
                }

                ctx.emit(&format!("br label %{}", end_lbl));
                ctx.emit_label(&next_lbl);
            }

            if let Some(def) = default {
                let dv = gen_expr(ctx, def);
                ctx.emit(&format!("store double {}, double* {}", dv, res_ptr));
            } else {
                ctx.emit(&format!("store double 0.0, double* {}", res_ptr));
            }
            ctx.emit(&format!("br label %{}", end_lbl));

            ctx.emit_label(&end_lbl);
            let r = ctx.tmp();
            ctx.emit(&format!("{} = load double, double* {}", r, res_ptr));
            r
        }

        // ── Math builtins ───────────────────────────────────────────────────
        Expr::Sqrt(a) => {
            let v = gen_expr(ctx, a);
            let r = ctx.tmp();
            ctx.emit(&format!("{} = call double @llvm.sqrt.f64(double {})", r, v));
            r
        }
        Expr::Sin(a) => {
            let v = gen_expr(ctx, a);
            let r = ctx.tmp();
            ctx.emit(&format!("{} = call double @llvm.sin.f64(double {})", r, v));
            r
        }
        Expr::Cos(a) => {
            let v = gen_expr(ctx, a);
            let r = ctx.tmp();
            ctx.emit(&format!("{} = call double @llvm.cos.f64(double {})", r, v));
            r
        }
        Expr::Exp(a) => {
            let v = gen_expr(ctx, a);
            let r = ctx.tmp();
            ctx.emit(&format!("{} = call double @llvm.exp.f64(double {})", r, v));
            r
        }
        Expr::Log(base, val) => {
            let bv = gen_expr(ctx, base);
            let vv = gen_expr(ctx, val);
            let ln_v = ctx.tmp(); ctx.emit(&format!("{} = call double @llvm.log.f64(double {})", ln_v, vv));
            let ln_b = ctx.tmp(); ctx.emit(&format!("{} = call double @llvm.log.f64(double {})", ln_b, bv));
            let r = ctx.tmp(); ctx.emit(&format!("{} = fdiv double {}, {}", r, ln_v, ln_b));
            r
        }
        Expr::Rand => {
            // Seed once, then call rand() from libc
            let seeded = ctx.tmp();
            ctx.emit(&format!("{} = load i1, i1* @.rand_seeded", seeded));
            let seed_lbl = ctx.lbl("rand_seed");
            let call_lbl = ctx.lbl("rand_call");
            ctx.emit(&format!("br i1 {}, label %{}, label %{}", seeded, call_lbl, seed_lbl));

            ctx.emit_label(&seed_lbl);
            let t = ctx.tmp();
            ctx.emit(&format!("{} = call i64 @time(i64* null)", t));
            let t32 = ctx.tmp();
            ctx.emit(&format!("{} = trunc i64 {} to i32", t32, t));
            ctx.emit(&format!("call void @srand(i32 {})", t32));
            ctx.emit("store i1 true, i1* @.rand_seeded");
            ctx.emit(&format!("br label %{}", call_lbl));

            ctx.emit_label(&call_lbl);
            let ri = ctx.tmp();
            ctx.emit(&format!("{} = call i32 @rand()", ri));
            // Normalize to [0, 1): ri / RAND_MAX (2147483647)
            let rf = ctx.tmp();
            ctx.emit(&format!("{} = sitofp i32 {} to double", rf, ri));
            let r = ctx.tmp();
            ctx.emit(&format!("{} = fdiv double {}, 2.147483647e9", r, rf));
            r
        }
        Expr::PI => fmt_double(std::f64::consts::PI),
        Expr::E  => fmt_double(std::f64::consts::E),

        // ── Error node ──────────────────────────────────────────────────────
        Expr::Error => { "0.0".into() }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Helper: check if a class inherits from a target class
// ─────────────────────────────────────────────────────────────────────────────
fn class_inherits_from(
    classes: &HashMap<String, ClassLayout>,
    layout: &ClassLayout,
    target: &str,
) -> bool {
    let mut current = layout.parent_name.as_ref().map(|s| s.as_str());
    while let Some(parent) = current {
        if parent == target { return true; }
        current = classes.get(parent).and_then(|l| l.parent_name.as_ref().map(|s| s.as_str()));
    }
    false
}

// ─────────────────────────────────────────────────────────────────────────────
// Helper: collect free variables in an expression
// ─────────────────────────────────────────────────────────────────────────────
fn collect_free_vars(expr: &Spanned<Expr>, bound: &[String]) -> Vec<String> {
    let mut free = Vec::new();
    collect_free_vars_inner(&expr.node, bound, &mut free);
    // Deduplicate
    let mut seen = std::collections::HashSet::new();
    free.retain(|v| seen.insert(v.clone()));
    free
}

fn collect_free_vars_inner(expr: &Expr, bound: &[String], free: &mut Vec<String>) {
    match expr {
        Expr::Identifier(name) => {
            if !bound.contains(name) && name != "self" {
                free.push(name.clone());
            }
        }
        Expr::Binary(l, _, r) => {
            collect_free_vars_inner(&l.node, bound, free);
            collect_free_vars_inner(&r.node, bound, free);
        }
        Expr::Unary(_, e) => collect_free_vars_inner(&e.node, bound, free),
        Expr::If { cond, then_expr, else_expr } => {
            collect_free_vars_inner(&cond.node, bound, free);
            collect_free_vars_inner(&then_expr.node, bound, free);
            collect_free_vars_inner(&else_expr.node, bound, free);
        }
        Expr::While { cond, body } => {
            collect_free_vars_inner(&cond.node, bound, free);
            collect_free_vars_inner(&body.node, bound, free);
        }
        Expr::For { var, iterable, body } => {
            collect_free_vars_inner(&iterable.node, bound, free);
            let mut inner_bound: Vec<String> = bound.to_vec();
            inner_bound.push(var.clone());
            collect_free_vars_inner(&body.node, &inner_bound, free);
        }
        Expr::Block(exprs) => {
            for e in exprs { collect_free_vars_inner(&e.node, bound, free); }
        }
        Expr::Let { bindings, body } => {
            let mut inner_bound: Vec<String> = bound.to_vec();
            for (name, _, init) in bindings {
                collect_free_vars_inner(&init.node, &inner_bound, free);
                inner_bound.push(name.clone());
            }
            collect_free_vars_inner(&body.node, &inner_bound, free);
        }
        Expr::Assignment { target, value } => {
            if !bound.contains(target) { free.push(target.clone()); }
            collect_free_vars_inner(&value.node, bound, free);
        }
        Expr::Call { args, .. } => {
            for a in args { collect_free_vars_inner(&a.node, bound, free); }
        }
        Expr::BaseCall { args } => {
            for a in args { collect_free_vars_inner(&a.node, bound, free); }
        }
        Expr::MethodCall { obj, args, .. } => {
            collect_free_vars_inner(&obj.node, bound, free);
            for a in args { collect_free_vars_inner(&a.node, bound, free); }
        }
        Expr::AttributeAccess { obj, .. } => {
            collect_free_vars_inner(&obj.node, bound, free);
        }
        Expr::Instantiation { args, .. } => {
            for a in args { collect_free_vars_inner(&a.node, bound, free); }
        }
        Expr::Lambda { params, body, .. } => {
            let mut inner_bound: Vec<String> = bound.to_vec();
            for p in params { inner_bound.push(p.name.clone()); }
            collect_free_vars_inner(&body.node, &inner_bound, free);
        }
        Expr::Is(e, _) | Expr::As(e, _) => collect_free_vars_inner(&e.node, bound, free),
        Expr::VectorLiteral(elems) => {
            for e in elems { collect_free_vars_inner(&e.node, bound, free); }
        }
        Expr::VectorGenerator { expr: e, var, iterable } => {
            collect_free_vars_inner(&iterable.node, bound, free);
            let mut inner_bound: Vec<String> = bound.to_vec();
            inner_bound.push(var.clone());
            collect_free_vars_inner(&e.node, &inner_bound, free);
        }
        Expr::Indexing { obj, index } => {
            collect_free_vars_inner(&obj.node, bound, free);
            collect_free_vars_inner(&index.node, bound, free);
        }
        Expr::Match { expr: e, cases, default } => {
            collect_free_vars_inner(&e.node, bound, free);
            for c in cases {
                // Patterns can bind variables
                let mut inner_bound: Vec<String> = bound.to_vec();
                if let Pattern::Variable { name, .. } = &c.pattern {
                    inner_bound.push(name.clone());
                }
                collect_free_vars_inner(&c.expr.node, &inner_bound, free);
            }
            if let Some(d) = default {
                collect_free_vars_inner(&d.node, bound, free);
            }
        }
        Expr::Sqrt(a) | Expr::Sin(a) | Expr::Cos(a) | Expr::Exp(a) => {
            collect_free_vars_inner(&a.node, bound, free);
        }
        Expr::Log(a, b) => {
            collect_free_vars_inner(&a.node, bound, free);
            collect_free_vars_inner(&b.node, bound, free);
        }
        // Literals, constants – no free vars
        Expr::Number(_) | Expr::String(_) | Expr::Boolean(_) |
        Expr::Rand | Expr::PI | Expr::E | Expr::Error => {}
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Helpers: infer return types and resolve object class at compile time
// ─────────────────────────────────────────────────────────────────────────────

/// Heuristic: infer the return type name of a method body expression.
/// Uses attribute types for `self.attr` accesses, and literal/op analysis.
fn infer_return_type_from_body(body: &Expr, attr_types: &HashMap<String, String>) -> Option<String> {
    match body {
        Expr::String(_) => Some("String".into()),
        Expr::Number(_) | Expr::PI | Expr::E | Expr::Rand => Some("Number".into()),
        Expr::Boolean(_) => Some("Boolean".into()),
        Expr::Binary(_, Op::Concat, _) | Expr::Binary(_, Op::ConcatSpace, _) => Some("String".into()),
        Expr::Binary(_, op, _) if matches!(op, Op::Add | Op::Sub | Op::Mul | Op::Div | Op::Mod | Op::Pow) => Some("Number".into()),
        Expr::Binary(_, op, _) if matches!(op, Op::Eq | Op::Neq | Op::Lt | Op::Gt | Op::Le | Op::Ge | Op::And | Op::Or) => Some("Boolean".into()),
        Expr::Unary(UnOp::Neg, _) => Some("Number".into()),
        Expr::Unary(UnOp::Not, _) => Some("Boolean".into()),
        Expr::Sqrt(_) | Expr::Sin(_) | Expr::Cos(_) | Expr::Exp(_) | Expr::Log(_, _) => Some("Number".into()),
        Expr::Is(_, _) => Some("Boolean".into()),
        Expr::AttributeAccess { attribute, .. } => {
            attr_types.get(attribute.as_str()).cloned()
        }
        Expr::Identifier(name) => {
            // self.attr shorthand: look up in attr_types
            attr_types.get(name.as_str()).cloned()
        }
        Expr::Block(stmts) => {
            stmts.last().and_then(|s| infer_return_type_from_body(&s.node, attr_types))
        }
        Expr::If { then_expr, .. } => infer_return_type_from_body(&then_expr.node, attr_types),
        Expr::Let { body, .. } => infer_return_type_from_body(&body.node, attr_types),
        _ => None,
    }
}

/// Resolve the HULK class name of an expression at compile time.
/// Returns Some("ClassName") if determinable, None otherwise.
fn resolve_obj_class_from_expr(ctx: &Ctx, expr: &Expr) -> Option<String> {
    match expr {
        Expr::Instantiation { ty, .. } => Some(ty.clone()),
        Expr::Identifier(name) => {
            if let Some((_, vt)) = ctx.get_var(name) {
                if let ValTy::Obj(cls) = vt {
                    return Some(cls);
                }
            }
            // If inside a method and name is "self", return current class
            if name == "self" {
                return ctx.current_class.clone();
            }
            None
        }
        Expr::As(_, ty_name) => Some(ty_name.clone()),
        _ => None,
    }
}

/// Infer the correct ValTy for a variable being bound to an expression.
fn infer_val_ty(ctx: &Ctx, init_expr: &Expr) -> ValTy {
    match init_expr {
        Expr::Instantiation { ty, .. } => ValTy::Obj(ty.clone()),
        Expr::String(_) => ValTy::Str,
        Expr::Boolean(_) => ValTy::Bool,
        Expr::Number(_) | Expr::PI | Expr::E | Expr::Rand => ValTy::Num,
        Expr::Binary(_, Op::Concat, _) | Expr::Binary(_, Op::ConcatSpace, _) => ValTy::Str,
        Expr::Binary(_, op, _) if matches!(op, Op::Add | Op::Sub | Op::Mul | Op::Div | Op::Mod | Op::Pow) => ValTy::Num,
        Expr::Binary(_, op, _) if matches!(op, Op::Eq | Op::Neq | Op::Lt | Op::Gt | Op::Le | Op::Ge | Op::And | Op::Or) => ValTy::Bool,
        Expr::Unary(UnOp::Neg, _) => ValTy::Num,
        Expr::Unary(UnOp::Not, _) => ValTy::Bool,
        Expr::Sqrt(_) | Expr::Sin(_) | Expr::Cos(_) | Expr::Exp(_) | Expr::Log(_, _) => ValTy::Num,
        Expr::Is(_, _) => ValTy::Bool,
        Expr::As(_, ty_name) => ValTy::Obj(ty_name.clone()),
        Expr::Identifier(name) => {
            if let Some((_, vt)) = ctx.get_var(name) {
                return vt;
            }
            ValTy::Num
        }
        Expr::If { then_expr, .. } => infer_val_ty(ctx, &then_expr.node),
        Expr::Block(stmts) => {
            stmts.last().map_or(ValTy::Num, |s| infer_val_ty(ctx, &s.node))
        }
        Expr::Let { body, .. } => infer_val_ty(ctx, &body.node),
        Expr::Lambda { body, .. } => {
            let ret_ty = infer_val_ty(ctx, &body.node);
            ValTy::Fn(Box::new(ret_ty))
        }
        Expr::Call { func, .. } => {
            // Check named functions
            if let Some(ret_vty) = ctx.func_ret_types.get(func.as_str()) {
                return ret_vty.clone();
            }
            // Check lambdas
            if let Some((_, ValTy::Fn(ret_ty))) = ctx.get_var(func) {
                return *ret_ty;
            }
            ValTy::Num
        }
        _ => ValTy::Num,
    }
}

/// Convert a parameter's type annotation to a ValTy.
fn val_ty_from_annotation(ann: &Option<TypeAnnotation>) -> ValTy {
    match ann {
        Some(TypeAnnotation::Name(name)) => match name.as_str() {
            "String" => ValTy::Str,
            "Number" => ValTy::Num,
            "Boolean" => ValTy::Bool,
            other => ValTy::Obj(other.to_string()),
        },
        _ => ValTy::Num,
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Print helper – detects Number vs Boolean vs String from AST + class metadata
// ─────────────────────────────────────────────────────────────────────────────

/// Type hint result for an expression.
#[derive(Clone, Copy, Debug, PartialEq)]
enum ExprTyHint { Str, Num, Bool, Unknown }

/// Determine the likely runtime type of an expression using AST structure
/// and class attribute metadata.
fn expr_type_hint(ctx: &Ctx, expr: &Expr) -> ExprTyHint {
    match expr {
        // Literals
        Expr::String(_) => ExprTyHint::Str,
        Expr::Number(_) | Expr::PI | Expr::E | Expr::Rand => ExprTyHint::Num,
        Expr::Boolean(_) => ExprTyHint::Bool,

        // Binary ops
        Expr::Binary(_, Op::Concat, _) | Expr::Binary(_, Op::ConcatSpace, _) => ExprTyHint::Str,
        Expr::Binary(_, op, _) if matches!(op, Op::Add | Op::Sub | Op::Mul | Op::Div | Op::Mod | Op::Pow) => ExprTyHint::Num,
        Expr::Binary(_, op, _) if matches!(op, Op::Eq | Op::Neq | Op::Lt | Op::Gt | Op::Le | Op::Ge | Op::And | Op::Or) => ExprTyHint::Bool,

        // Unary
        Expr::Unary(UnOp::Neg, _) => ExprTyHint::Num,
        Expr::Unary(UnOp::Not, _) => ExprTyHint::Bool,

        // Math builtins
        Expr::Sqrt(_) | Expr::Sin(_) | Expr::Cos(_) | Expr::Exp(_) | Expr::Log(_, _) => ExprTyHint::Num,

        // Type checks
        Expr::Is(_, _) => ExprTyHint::Bool,

        // AttributeAccess: look up the attribute type in the resolved class first
        Expr::AttributeAccess { obj, attribute, .. } => {
            let obj_class = resolve_obj_class_from_expr(ctx, &obj.node);
            if let Some(ref cls) = obj_class {
                if let Some(layout) = ctx.classes.get(cls.as_str()) {
                    if let Some(type_name) = layout.attr_types.get(attribute.as_str()) {
                        return match type_name.as_str() {
                            "String" => ExprTyHint::Str,
                            "Number" => ExprTyHint::Num,
                            "Boolean" => ExprTyHint::Bool,
                            _ => ExprTyHint::Unknown,
                        };
                    }
                }
            }
            // Fallback: search all classes
            for (_, layout) in &ctx.classes {
                if let Some(type_name) = layout.attr_types.get(attribute.as_str()) {
                    return match type_name.as_str() {
                        "String" => ExprTyHint::Str,
                        "Number" => ExprTyHint::Num,
                        "Boolean" => ExprTyHint::Bool,
                        _ => ExprTyHint::Unknown,
                    };
                }
            }
            ExprTyHint::Unknown
        }

        // Method calls: check method_ret_types from resolved class or any class
        Expr::MethodCall { obj, method, .. } => {
            // First try the resolved class of the object
            let obj_class = resolve_obj_class_from_expr(ctx, &obj.node);
            if let Some(ref cls) = obj_class {
                if let Some(layout) = ctx.classes.get(cls.as_str()) {
                    if let Some(ret_type) = layout.method_ret_types.get(method.as_str()) {
                        return match ret_type.as_str() {
                            "String" => ExprTyHint::Str,
                            "Number" => ExprTyHint::Num,
                            "Boolean" => ExprTyHint::Bool,
                            _ => ExprTyHint::Unknown,
                        };
                    }
                }
            }
            // Fallback: search all classes
            for (_, layout) in &ctx.classes {
                if let Some(ret_type) = layout.method_ret_types.get(method.as_str()) {
                    return match ret_type.as_str() {
                        "String" => ExprTyHint::Str,
                        "Number" => ExprTyHint::Num,
                        "Boolean" => ExprTyHint::Bool,
                        _ => ExprTyHint::Unknown,
                    };
                }
            }
            ExprTyHint::Unknown
        }

        // Control flow
        Expr::If { then_expr, else_expr, .. } => {
            let t = expr_type_hint(ctx, &then_expr.node);
            let e = expr_type_hint(ctx, &else_expr.node);
            if t == e { t } else if t != ExprTyHint::Unknown { t } else { e }
        }
        Expr::Block(stmts) => {
            stmts.last().map_or(ExprTyHint::Unknown, |s| expr_type_hint(ctx, &s.node))
        }
        Expr::Let { body, .. } => expr_type_hint(ctx, &body.node),

        // Identifiers: check the tracked val_types
        Expr::Identifier(name) => {
            match ctx.get_var(name).map(|(_, vt)| vt) {
                Some(ValTy::Str) => ExprTyHint::Str,
                Some(ValTy::Bool) => ExprTyHint::Bool,
                Some(ValTy::Num) => ExprTyHint::Num,
                _ => ExprTyHint::Unknown,
            }
        }

        // Function calls: check return type from functions or val_types (for lambdas)
        Expr::Call { func, .. } => {
            // First check named functions with return type annotations
            if let Some(ret_vty) = ctx.func_ret_types.get(func.as_str()) {
                return match ret_vty {
                    ValTy::Str => ExprTyHint::Str,
                    ValTy::Bool => ExprTyHint::Bool,
                    ValTy::Num => ExprTyHint::Num,
                    _ => ExprTyHint::Unknown,
                };
            }
            // Then check if it's a lambda stored in val_types
            match ctx.get_var(func).map(|(_, vt)| vt) {
                Some(ValTy::Fn(ret_ty)) => match *ret_ty {
                    ValTy::Str => ExprTyHint::Str,
                    ValTy::Bool => ExprTyHint::Bool,
                    ValTy::Num => ExprTyHint::Num,
                    _ => ExprTyHint::Unknown,
                },
                Some(ValTy::Str) => ExprTyHint::Str,
                Some(ValTy::Bool) => ExprTyHint::Bool,
                Some(ValTy::Num) => ExprTyHint::Num,
                _ => ExprTyHint::Unknown,
            }
        }

        _ => ExprTyHint::Unknown,
    }
}

/// Given a compiled double value and its AST node, return an `i8*` string pointer.
/// If the expression is already a string, bitcast the double to a pointer.
/// If it's a number, call @__hulk_num_to_str.  If a bool, call @__hulk_bool_to_str.
/// If unknown type, call @__hulk_to_str (runtime auto-detect).
fn gen_to_str_ptr(ctx: &mut Ctx, expr: &Expr, val: &str) -> String {
    let hint = expr_type_hint(ctx, expr);
    match hint {
        ExprTyHint::Str => {
            ctx.decode_ptr(val, "i8*")
        }
        ExprTyHint::Bool => {
            let p = ctx.tmp();
            ctx.emit(&format!("{} = call i8* @__hulk_bool_to_str(double {})", p, val));
            p
        }
        ExprTyHint::Num => {
            let p = ctx.tmp();
            ctx.emit(&format!("{} = call i8* @__hulk_num_to_str(double {})", p, val));
            p
        }
        ExprTyHint::Unknown => {
            // Unknown type at compile time — runtime auto-detect (string pointer vs number)
            let p = ctx.tmp();
            ctx.emit(&format!("{} = call i8* @__hulk_to_str(double {})", p, val));
            p
        }
    }
}

fn gen_print(ctx: &mut Ctx, args: &[Spanned<Expr>]) {
    if let Some(arg) = args.first() {
        let hint = expr_type_hint(ctx, &arg.node);
        let val = gen_expr(ctx, arg);

        match hint {
            ExprTyHint::Str => {
                let p = ctx.decode_ptr(&val, "i8*");
                ctx.emit(&format!("call i32 @puts(i8* {})", p));
            }
            ExprTyHint::Bool => {
                let cb = ctx.tmp(); ctx.emit(&format!("{} = fcmp one double {}, 0.0", cb, val));
                let ts = ctx.tmp();
                ctx.emit(&format!("{} = select i1 {}, i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.true_s, i64 0, i64 0), i8* getelementptr inbounds ([6 x i8], [6 x i8]* @.false_s, i64 0, i64 0)", ts, cb));
                ctx.emit(&format!("call i32 @puts(i8* {})", ts));
            }
            ExprTyHint::Num => {
                ctx.emit(&format!("call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.fmt_num, i64 0, i64 0), double {})", val));
                ctx.emit("call i32 @puts(i8* getelementptr inbounds ([1 x i8], [1 x i8]* @.empty_s, i64 0, i64 0))");
            }
            ExprTyHint::Unknown => {
                ctx.emit(&format!("call void @__hulk_print_val(double {})", val));
            }
        }
    }
}
