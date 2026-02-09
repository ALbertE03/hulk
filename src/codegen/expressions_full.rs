use std::collections::HashMap;
use crate::ast::nodes::{Expr, Spanned, Op, UnOp, TypeAnnotation, Pattern};
use super::context::{Ctx, ClassLayout};
use super::utils::{fmt_double, ValTy, val_ty_from_annotation};
use super::functions::mangle_fn;

// Este archivo contiene el código completo de generación de expresiones
// que será usado para reemplazar expressions.rs
