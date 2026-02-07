use std::fmt;
use crate::ast::nodes::*;

impl fmt::Display for TypeAnnotation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeAnnotation::Name(s) => write!(f, "{}", s),
            TypeAnnotation::Function { params, return_type } => {
                write!(f, "(")?;
                for (i, p) in params.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", p)?;
                }
                write!(f, ") -> {}", return_type)
            }
            TypeAnnotation::Iterable(ty) => write!(f, "{}*", ty),
        }
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for decl in &self.declarations {
            writeln!(f, "{}", decl)?;
        }
        write!(f, "{}", self.expr.node)
    }
}

impl fmt::Display for Declaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Declaration::Function(func) => write!(f, "{}", func),
            Declaration::Type(ty) => write!(f, "{}", ty),
            Declaration::Protocol(proto) => write!(f, "{}", proto),
            Declaration::Macro(macro_decl) => write!(f, "{}", macro_decl),
        }
    }
}

impl fmt::Display for FunctionDecl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "function {}(", self.name)?;
        for (i, param) in self.params.iter().enumerate() {
            if i > 0 { write!(f, ", ")?; }
            write!(f, "{}", param)?;
        }
        write!(f, ")")?;
        if let Some(ret_type) = &self.return_type {
            write!(f, ": {}", ret_type)?;
        }
        write!(f, " => {}", self.body.node)?;
        writeln!(f, ";")
    }
}

impl fmt::Display for TypeDecl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "type {}", self.name)?;
        if !self.params.is_empty() {
            write!(f, "(")?;
            for (i, param) in self.params.iter().enumerate() {
                if i > 0 { write!(f, ", ")?; }
                write!(f, "{}", param)?;
            }
            write!(f, ")")?;
        }
        if let Some(parent) = &self.parent {
            write!(f, " inherits {}", parent)?;
        }
        writeln!(f, " {{")?;
        for attr in &self.attributes {
            writeln!(f, "    {}", attr)?;
        }
        for method in &self.methods {
            writeln!(f, "    {}", method)?;
        }
        writeln!(f, "}}")
    }
}

impl fmt::Display for ProtocolDecl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "protocol {}", self.name)?;
        if let Some(parent) = &self.parent {
            write!(f, " extends {}", parent)?;
        }
        writeln!(f, " {{")?;
        for method in &self.methods {
            writeln!(f, "    {};", method)?;
        }
        writeln!(f, "}}")
    }
}

impl fmt::Display for Param {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)?;
        if let Some(ty) = &self.type_annotation {
            write!(f, ": {}", ty)?;
        }
        Ok(())
    }
}

impl fmt::Display for TypeInit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}(", self.name)?;
        for (i, arg) in self.args.iter().enumerate() {
            if i > 0 { write!(f, ", ")?; }
            write!(f, "{}", arg.node)?;
        }
        write!(f, ")")
    }
}

impl fmt::Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)?;
        if let Some(ty) = &self.type_annotation {
            write!(f, ": {}", ty)?;
        }
        write!(f, " = {};", self.init.node)
    }
}

impl fmt::Display for MethodSignature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}(", self.name)?;
        for (i, param) in self.params.iter().enumerate() {
            if i > 0 { write!(f, ", ")?; }
            write!(f, "{}", param)?;
        }
        write!(f, "): {}", self.return_type)
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Number(n) => write!(f, "{}", n),
            Expr::String(s) => write!(f, "\"{}\"", s), // TODO: Escapar correctamente
            Expr::Boolean(b) => write!(f, "{}", b),
            Expr::Identifier(s) => write!(f, "{}", s),
            Expr::Binary(lhs, op, rhs) => write!(f, "({} {} {})", lhs.node, op, rhs.node),
            Expr::Unary(op, expr) => write!(f, "({}{})", op, expr.node),
            Expr::If { cond, then_expr, else_expr } => {
                write!(f, "if ({}) {} else {}", cond.node, then_expr.node, else_expr.node)
            },
            Expr::While { cond, body } => write!(f, "while ({}) {}", cond.node, body.node),
            Expr::For { var, iterable, body } => write!(f, "for ({} in {}) {}", var, iterable.node, body.node),
            Expr::Block(exprs) => {
                writeln!(f, "{{")?;
                for expr in exprs {
                    writeln!(f, "    {};", expr.node)?;
                }
                write!(f, "}}")
            },
            Expr::Let { bindings, body } => {
                write!(f, "let ")?;
                for (i, (name, ty, val)) in bindings.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", name)?;
                    if let Some(t) = ty {
                        write!(f, ": {}", t)?;
                    }
                    write!(f, " = {}", val.node)?;
                }
                write!(f, " in {}", body.node)
            },
            Expr::Assignment { target, value } => write!(f, "{} := {}", target, value.node),
            Expr::AttributeAssignment { obj, attribute, value } => write!(f, "{}.{} := {}", obj.node, attribute, value.node),
            Expr::Call { func, args } => {
                write!(f, "{}(", func)?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", arg.node)?;
                }
                write!(f, ")")
            },
            Expr::BaseCall { args } => {
                write!(f, "base(")?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", arg.node)?;
                }
                write!(f, ")")
            },
            Expr::MethodCall { obj, method, args } => {
                write!(f, "{}.{}(", obj.node, method)?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", arg.node)?;
                }
                write!(f, ")")
            },
            Expr::AttributeAccess { obj, attribute } => {
                write!(f, "{}.{}", obj.node, attribute)
            },
            Expr::Instantiation { ty, args } => {
                write!(f, "new {}(", ty)?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", arg.node)?;
                }
                write!(f, ")")
            },
            Expr::Is(expr, ty) => write!(f, "({} is {})", expr.node, ty),
            Expr::As(expr, ty) => write!(f, "({} as {})", expr.node, ty),
            Expr::VectorLiteral(exprs) => {
                write!(f, "[")?;
                for (i, expr) in exprs.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", expr.node)?;
                }
                write!(f, "]")
            },
            Expr::VectorGenerator { expr, var, iterable } => {
                write!(f, "[{} | {} in {}]", expr.node, var, iterable.node)
            },
            Expr::Lambda { params, return_type, body } => {
                write!(f, "(")?;
                for (i, p) in params.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", p)?;
                }
                write!(f, ")")?;
                if let Some(ret) = return_type {
                    write!(f, ": {}", ret)?;
                }
                write!(f, " => {}", body.node)
            },
            Expr::Indexing { obj, index } => write!(f, "{}[{}]", obj.node, index.node),
            Expr::Sqrt(e) => write!(f, "sqrt({})", e.node),
            Expr::Sin(e) => write!(f, "sin({})", e.node),
            Expr::Cos(e) => write!(f, "cos({})", e.node),
            Expr::Exp(e) => write!(f, "exp({})", e.node),
            Expr::Log(b, v) => write!(f, "log({}, {})", b.node, v.node),
            Expr::Rand => write!(f, "rand()"),
            Expr::PI => write!(f, "PI"),
            Expr::E => write!(f, "E"),
            Expr::Match { expr, cases, default } => {
                write!(f, "match({}) {{\n", expr.node)?;
                for case in cases {
                    write!(f, "  case {} => {};\n", case.pattern, case.expr.node)?;
                }
                if let Some(def) = default {
                    write!(f, "  default => {};\n", def.node)?;
                }
                write!(f, "}}")
            },
            Expr::Error => write!(f, "<error>"),
        }
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Op::Add => write!(f, "+"),
            Op::Sub => write!(f, "-"),
            Op::Mul => write!(f, "*"),
            Op::Div => write!(f, "/"),
            Op::Mod => write!(f, "%"),
            Op::Pow => write!(f, "^"),
            Op::Eq => write!(f, "=="),
            Op::Neq => write!(f, "!="),
            Op::Lt => write!(f, "<"),
            Op::Gt => write!(f, ">"),
            Op::Le => write!(f, "<="),
            Op::Ge => write!(f, ">="),
            Op::And => write!(f, "&"),
            Op::Or => write!(f, "|"),
            Op::Concat => write!(f, "@"),
            Op::ConcatSpace => write!(f, "@@"),
        }
    }
}

impl fmt::Display for UnOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnOp::Neg => write!(f, "-"),
            UnOp::Not => write!(f, "!"),
        }
    }
}

impl fmt::Display for MacroDecl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "def {}(", self.name)?;
        for (i, param) in self.params.iter().enumerate() {
            if i > 0 { write!(f, ", ")?; }
            write!(f, "{}", param)?;
        }
        write!(f, ")")?;
        if let Some(ret_type) = &self.return_type {
            write!(f, ": {}", ret_type)?;
        }
        write!(f, " => {}", self.body.node)
    }
}

impl fmt::Display for MacroParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MacroParam::Normal { name, type_annotation } => {
                write!(f, "{}: {}", name, type_annotation)
            }
            MacroParam::Symbolic { name, type_annotation } => {
                write!(f, "@{}: {}", name, type_annotation)
            }
            MacroParam::Placeholder { name, type_annotation } => {
                write!(f, "${}: {}", name, type_annotation)
            }
            MacroParam::Body { name, type_annotation } => {
                write!(f, "*{}: {}", name, type_annotation)
            }
        }
    }
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Pattern::Literal(expr) => write!(f, "{}", expr),
            Pattern::Variable { name, type_annotation } => {
                if let Some(ty) = type_annotation {
                    write!(f, "{}: {}", name, ty)
                } else {
                    write!(f, "{}", name)
                }
            }
            Pattern::Binary { left, op, right } => {
                write!(f, "({} {} {})", left, op, right)
            }
            Pattern::Unary { op, operand } => {
                write!(f, "{}{}", op, operand)
            }
            Pattern::Call { func, args } => {
                write!(f, "{}(", func)?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", arg)?;
                }
                write!(f, ")")
            }
            Pattern::Wildcard => write!(f, "_"),
        }
    }
}
