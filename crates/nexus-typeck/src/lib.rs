use std::collections::HashMap;
use nexus_parser::ast::*;

// ── Type context ─────────────────────────────────────────────

#[derive(Clone)]
struct Var {
    ty: Ty,
    mutable: bool,
}

struct TypeCtx {
    variables: HashMap<String, Var>,
    functions: HashMap<String, (Vec<Ty>, Ty)>, // (param_types, return_ty)
    diagnostics: Vec<String>,
}

impl TypeCtx {
    fn new() -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
            diagnostics: Vec::new(),
        }
    }
}

// ── Public entry point ───────────────────────────────────────

pub fn type_check(program: &Program) -> Vec<String> {
    let mut ctx = TypeCtx::new();
    for stmt in &program.stmts {
        check_stmt(stmt, &mut ctx);
    }
    ctx.diagnostics
}

// ── Statement checking ───────────────────────────────────────

fn check_stmt(stmt: &Stmt, ctx: &mut TypeCtx) {
    match stmt {
        Stmt::Let { name, mutable, ty_annotation, expr, .. } => {
            let expr_ty = infer_expr(expr, ctx);

            // Validate annotation if present
            if let Some(ann) = ty_annotation {
                if !types_compatible(ann, &expr_ty) {
                    ctx.diagnostics.push(format!(
                        "Type mismatch in 'let {}': annotated {}, found {}",
                        name, ann.display(), expr_ty.display()
                    ));
                }
            }

            let ty = if let Some(ann) = ty_annotation {
                ann.clone()
            } else {
                expr_ty
            };

            ctx.variables.insert(name.clone(), Var { ty, mutable: *mutable });
        }

        Stmt::Assign { name, expr, .. } => {
            let expr_ty = infer_expr(expr, ctx);
            if let Some(var) = ctx.variables.get(name) {
                if !var.mutable {
                    ctx.diagnostics.push(format!(
                        "Cannot assign to immutable variable '{}'", name
                    ));
                }
                if !types_compatible(&var.ty, &expr_ty) {
                    ctx.diagnostics.push(format!(
                        "Type mismatch in assignment to '{}': expected {}, found {}",
                        name, var.ty.display(), expr_ty.display()
                    ));
                }
            } else {
                ctx.diagnostics.push(format!("Undefined variable '{}' in assignment", name));
            }
        }

        Stmt::Expr(expr, _) => {
            infer_expr(expr, ctx);
        }

        Stmt::Return(expr, _) => {
            infer_expr(expr, ctx);
        }

        Stmt::FnDef { name, params, return_ty, body, .. } => {
            let param_tys: Vec<Ty> = params.iter().map(|p| p.ty.clone()).collect();
            ctx.functions.insert(name.clone(), (param_tys.clone(), return_ty.clone()));

            // Also register the function name as a variable of function type
            ctx.variables.insert(name.clone(), Var {
                ty: Ty::Fn(param_tys.clone(), Box::new(return_ty.clone())),
                mutable: false,
            });

            // Type check body in a sub-context with params
            let mut body_ctx = TypeCtx {
                variables: ctx.variables.clone(),
                functions: ctx.functions.clone(),
                diagnostics: Vec::new(),
            };

            for param in params {
                body_ctx.variables.insert(param.name.clone(), Var {
                    ty: param.ty.clone(),
                    mutable: false,
                });
            }

            for s in body {
                check_stmt(s, &mut body_ctx);
            }

            ctx.diagnostics.append(&mut body_ctx.diagnostics);
        }

        Stmt::Block(stmts, _) => {
            for s in stmts {
                check_stmt(s, ctx);
            }
        }

        Stmt::If { cond, then_branch, else_branch, .. } => {
            let cond_ty = infer_expr(cond, ctx);
            if !matches!(cond_ty, Ty::Bool | Ty::Error | Ty::Unknown) {
                ctx.diagnostics.push(format!(
                    "If condition must be bool, found {}", cond_ty.display()
                ));
            }
            for s in then_branch {
                check_stmt(s, ctx);
            }
            if let Some(else_stmts) = else_branch {
                for s in else_stmts {
                    check_stmt(s, ctx);
                }
            }
        }

        Stmt::While { cond, body, .. } => {
            let cond_ty = infer_expr(cond, ctx);
            if !matches!(cond_ty, Ty::Bool | Ty::Error | Ty::Unknown) {
                ctx.diagnostics.push(format!(
                    "While condition must be bool, found {}", cond_ty.display()
                ));
            }
            for s in body {
                check_stmt(s, ctx);
            }
        }
    }
}

// ── Expression type inference ────────────────────────────────

fn infer_expr(expr: &Expr, ctx: &mut TypeCtx) -> Ty {
    match expr {
        Expr::Literal(lit, _) => match lit {
            Literal::Number(n) => {
                // If it has a fractional part, it's F64; otherwise I32
                if n.fract() == 0.0 && *n >= i32::MIN as f64 && *n <= i32::MAX as f64 {
                    Ty::I32
                } else {
                    Ty::F64
                }
            }
            Literal::String(_) => Ty::Str,
            Literal::Bool(_) => Ty::Bool,
        },

        Expr::Variable(name, _) => {
            if let Some(var) = ctx.variables.get(name) {
                var.ty.clone()
            } else {
                ctx.diagnostics.push(format!("Undefined variable '{}'", name));
                Ty::Error
            }
        }

        Expr::Binary { left, op, right, .. } => {
            let lty = infer_expr(left, ctx);
            let rty = infer_expr(right, ctx);

            match op {
                BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div => {
                    if matches!((&lty, &rty), (Ty::I32, Ty::I32)) {
                        Ty::I32
                    } else if matches!((&lty, &rty), (Ty::F64, Ty::F64) | (Ty::I32, Ty::F64) | (Ty::F64, Ty::I32)) {
                        Ty::F64
                    } else if matches!((&lty, &rty), (Ty::Str, Ty::Str)) && *op == BinOp::Add {
                        Ty::Str
                    } else if matches!(lty, Ty::Error) || matches!(rty, Ty::Error) {
                        Ty::Error
                    } else {
                        ctx.diagnostics.push(format!(
                            "Cannot apply '{}' to {} and {}",
                            op.display(), lty.display(), rty.display()
                        ));
                        Ty::Error
                    }
                }

                BinOp::Eq | BinOp::NotEq => Ty::Bool,

                BinOp::Lt | BinOp::Gt | BinOp::Le | BinOp::Ge => {
                    if matches!((&lty, &rty), (Ty::I32, Ty::I32) | (Ty::F64, Ty::F64)) {
                        Ty::Bool
                    } else if matches!(lty, Ty::Error) || matches!(rty, Ty::Error) {
                        Ty::Bool
                    } else {
                        ctx.diagnostics.push(format!(
                            "Comparison '{}' requires numbers of same type, found {} and {}",
                            op.display(), lty.display(), rty.display()
                        ));
                        Ty::Error
                    }
                }

                BinOp::And | BinOp::Or => {
                    if matches!((&lty, &rty), (Ty::Bool, Ty::Bool)) {
                        Ty::Bool
                    } else if matches!(lty, Ty::Error) || matches!(rty, Ty::Error) {
                        Ty::Error
                    } else {
                        ctx.diagnostics.push(format!(
                            "'{}' requires bool operands, found {} and {}",
                            op.display(), lty.display(), rty.display()
                        ));
                        Ty::Error
                    }
                }
            }
        }

        Expr::Unary { op, expr: inner, .. } => {
            let ty = infer_expr(inner, ctx);
            match op {
                UnaryOp::Not => {
                    if matches!(ty, Ty::Bool | Ty::Error) {
                        Ty::Bool
                    } else {
                        ctx.diagnostics.push(format!(
                            "'!' requires bool, found {}", ty.display()
                        ));
                        Ty::Error
                    }
                }
                UnaryOp::Neg => {
                    if matches!(ty, Ty::I32 | Ty::F64 | Ty::Error) {
                        ty
                    } else {
                        ctx.diagnostics.push(format!(
                            "'-' requires number, found {}", ty.display()
                        ));
                        Ty::Error
                    }
                }
            }
        }

        Expr::Call { callee, args, .. } => {
            if let Expr::Variable(name, _) = &**callee {
                // Builtins
                if name == "print" {
                    for arg in args {
                        infer_expr(arg, ctx);
                    }
                    return Ty::Unit;
                }
                if name == "len" {
                    if args.len() != 1 {
                        ctx.diagnostics.push(format!(
                            "'len' expects 1 arg, got {}", args.len()
                        ));
                    } else {
                        let arg_ty = infer_expr(&args[0], ctx);
                        if !matches!(arg_ty, Ty::Array(_) | Ty::Str | Ty::Error | Ty::Unknown) {
                            ctx.diagnostics.push(format!(
                                "'len' requires array or str, found {}", arg_ty.display()
                            ));
                        }
                    }
                    return Ty::I32;
                }
                if name == "push" {
                    if args.len() != 2 {
                        ctx.diagnostics.push(format!(
                            "'push' expects 2 args (array, element), got {}", args.len()
                        ));
                    } else {
                        let arr_ty = infer_expr(&args[0], ctx);
                        let elem_ty = infer_expr(&args[1], ctx);
                        if let Ty::Array(inner) = &arr_ty {
                            if !types_compatible(inner, &elem_ty) {
                                ctx.diagnostics.push(format!(
                                    "'push' element type mismatch: expected {}, found {}",
                                    inner.display(), elem_ty.display()
                                ));
                            }
                        } else if !matches!(arr_ty, Ty::Error | Ty::Unknown) {
                            ctx.diagnostics.push(format!(
                                "'push' requires array as first arg, found {}", arr_ty.display()
                            ));
                        }
                    }
                    return Ty::Unit;
                }

                if let Some((param_tys, ret_ty)) = ctx.functions.get(name).cloned() {
                    if args.len() != param_tys.len() {
                        ctx.diagnostics.push(format!(
                            "Function '{}' expects {} args, got {}",
                            name, param_tys.len(), args.len()
                        ));
                    } else {
                        for (i, (arg, expected)) in args.iter().zip(param_tys.iter()).enumerate() {
                            let arg_ty = infer_expr(arg, ctx);
                            if !types_compatible(expected, &arg_ty) {
                                ctx.diagnostics.push(format!(
                                    "Arg {} of '{}': expected {}, found {}",
                                    i + 1, name, expected.display(), arg_ty.display()
                                ));
                            }
                        }
                    }
                    ret_ty
                } else {
                    ctx.diagnostics.push(format!("Undefined function '{}'", name));
                    Ty::Error
                }
            } else {
                // For now, only support calling named functions
                ctx.diagnostics.push("Can only call named functions".into());
                Ty::Error
            }
        }

        Expr::ArrayLit { elements, .. } => {
            if elements.is_empty() {
                Ty::Array(Box::new(Ty::Unknown))
            } else {
                let first_ty = infer_expr(&elements[0], ctx);
                for elem in &elements[1..] {
                    let elem_ty = infer_expr(elem, ctx);
                    if !types_compatible(&first_ty, &elem_ty) {
                        ctx.diagnostics.push(format!(
                            "Array element type mismatch: expected {}, found {}",
                            first_ty.display(), elem_ty.display()
                        ));
                    }
                }
                Ty::Array(Box::new(first_ty))
            }
        }

        Expr::Index { array, index, .. } => {
            let arr_ty = infer_expr(array, ctx);
            let idx_ty = infer_expr(index, ctx);

            if !matches!(idx_ty, Ty::I32 | Ty::Error | Ty::Unknown) {
                ctx.diagnostics.push(format!(
                    "Array index must be i32, found {}", idx_ty.display()
                ));
            }

            match arr_ty {
                Ty::Array(inner) => *inner,
                Ty::Error | Ty::Unknown => Ty::Error,
                other => {
                    ctx.diagnostics.push(format!(
                        "Cannot index into type {}", other.display()
                    ));
                    Ty::Error
                }
            }
        }

        Expr::Error(_) => Ty::Error,
    }
}

// ── Type compatibility ───────────────────────────────────────

fn types_compatible(expected: &Ty, actual: &Ty) -> bool {
    match (expected, actual) {
        (a, b) if a == b => true,
        (Ty::Error, _) | (_, Ty::Error) => true,
        (Ty::Unknown, _) | (_, Ty::Unknown) => true,
        (Ty::Array(a), Ty::Array(b)) => types_compatible(a, b),
        (Ty::Fn(p1, r1), Ty::Fn(p2, r2)) => {
            p1.len() == p2.len()
                && p1.iter().zip(p2).all(|(a, b)| types_compatible(a, b))
                && types_compatible(r1, r2)
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nexus_lexer::Lexer;
    use nexus_parser::Parser;

    fn check(src: &str) -> Vec<String> {
        let (tokens, _) = Lexer::new(src).tokenize();
        let (program, _) = Parser::new(tokens).parse();
        type_check(&program)
    }

    #[test]
    fn valid_arithmetic() {
        let diags = check("let x = 1 + 2;");
        assert!(diags.is_empty(), "{:?}", diags);
    }

    #[test]
    fn immutable_assign() {
        let diags = check("let x = 1; x = 2;");
        assert!(diags.iter().any(|d| d.contains("immutable")), "{:?}", diags);
    }

    #[test]
    fn mutable_assign_ok() {
        let diags = check("let mut x = 1; x = 2;");
        assert!(diags.is_empty(), "{:?}", diags);
    }

    #[test]
    fn type_mismatch_assign() {
        let diags = check("let mut x = 1; x = true;");
        assert!(diags.iter().any(|d| d.contains("Type mismatch")), "{:?}", diags);
    }

    #[test]
    fn undefined_variable() {
        let diags = check("let x = y;");
        assert!(diags.iter().any(|d| d.contains("Undefined variable")), "{:?}", diags);
    }

    #[test]
    fn function_type_check() {
        let diags = check("fn add(a: i32, b: i32) -> i32 { return a + b; } let x = add(1, 2);");
        assert!(diags.is_empty(), "{:?}", diags);
    }

    #[test]
    fn function_wrong_arg_count() {
        let diags = check("fn f(a: i32) -> i32 { return a; } f(1, 2);");
        assert!(diags.iter().any(|d| d.contains("expects 1 args")), "{:?}", diags);
    }

    #[test]
    fn if_non_bool_condition() {
        let diags = check("if 42 { let x = 1; }");
        assert!(diags.iter().any(|d| d.contains("must be bool")), "{:?}", diags);
    }

    #[test]
    fn unary_neg() {
        let diags = check("let x = -5;");
        assert!(diags.is_empty(), "{:?}", diags);
    }

    #[test]
    fn unary_not() {
        let diags = check("let x = !true;");
        assert!(diags.is_empty(), "{:?}", diags);
    }
}
