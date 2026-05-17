use nexus_span::Span;

// ── Types ────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Ty {
    I32,
    F64,
    Bool,
    Str,
    Unit,
    Never,
    Unknown,
    Error,
    Array(Box<Ty>),
    Fn(Vec<Ty>, Box<Ty>),
}

impl std::fmt::Display for Ty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.display())
    }
}

impl Ty {
    pub fn display(&self) -> String {
        match self {
            Ty::I32 => "i32".into(),
            Ty::F64 => "f64".into(),
            Ty::Bool => "bool".into(),
            Ty::Str => "str".into(),
            Ty::Unit => "()".into(),
            Ty::Never => "!".into(),
            Ty::Unknown => "?".into(),
            Ty::Error => "<error>".into(),
            Ty::Array(inner) => format!("[{}]", inner.display()),
            Ty::Fn(params, ret) => {
                let p = params.iter().map(|t| t.display()).collect::<Vec<_>>().join(", ");
                format!("fn({}) -> {}", p, ret.display())
            }
        }
    }
}

// ── AST nodes ────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Program {
    pub stmts: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub ty: Ty,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Let {
        name: String,
        mutable: bool,
        ty_annotation: Option<Ty>,
        expr: Box<Expr>,
        span: Span,
    },
    Assign {
        name: String,
        expr: Box<Expr>,
        span: Span,
    },
    Expr(Box<Expr>, Span),
    Return(Box<Expr>, Span),
    FnDef {
        name: String,
        params: Vec<Param>,
        return_ty: Ty,
        body: Vec<Stmt>,
        span: Span,
    },
    Block(Vec<Stmt>, Span),
    If {
        cond: Box<Expr>,
        then_branch: Vec<Stmt>,
        else_branch: Option<Vec<Stmt>>,
        span: Span,
    },
    While {
        cond: Box<Expr>,
        body: Vec<Stmt>,
        span: Span,
    },
}

impl Stmt {
    pub fn span(&self) -> Span {
        match self {
            Stmt::Let { span, .. }
            | Stmt::Assign { span, .. }
            | Stmt::Expr(_, span)
            | Stmt::Return(_, span)
            | Stmt::FnDef { span, .. }
            | Stmt::Block(_, span)
            | Stmt::If { span, .. }
            | Stmt::While { span, .. } => *span,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal, Span),
    Variable(String, Span),
    Binary {
        left: Box<Expr>,
        op: BinOp,
        right: Box<Expr>,
        span: Span,
    },
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
        span: Span,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
        span: Span,
    },
    ArrayLit {
        elements: Vec<Expr>,
        span: Span,
    },
    Index {
        array: Box<Expr>,
        index: Box<Expr>,
        span: Span,
    },
    Error(Span),
}

impl Expr {
    pub fn span(&self) -> Span {
        match self {
            Expr::Literal(_, s)
            | Expr::Variable(_, s)
            | Expr::Binary { span: s, .. }
            | Expr::Unary { span: s, .. }
            | Expr::Call { span: s, .. }
            | Expr::ArrayLit { span: s, .. }
            | Expr::Index { span: s, .. }
            | Expr::Error(s) => *s,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Number(f64),
    String(String),
    Bool(bool),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    NotEq,
    Lt,
    Gt,
    Le,
    Ge,
    And,
    Or,
}

impl std::fmt::Display for BinOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.display())
    }
}

impl BinOp {
    pub fn display(&self) -> &'static str {
        match self {
            BinOp::Add => "+",
            BinOp::Sub => "-",
            BinOp::Mul => "*",
            BinOp::Div => "/",
            BinOp::Eq => "==",
            BinOp::NotEq => "!=",
            BinOp::Lt => "<",
            BinOp::Gt => ">",
            BinOp::Le => "<=",
            BinOp::Ge => ">=",
            BinOp::And => "&&",
            BinOp::Or => "||",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOp {
    Not,
    Neg,
}

// ── Pretty printer ───────────────────────────────────────────

pub fn pretty_print(program: &Program) -> String {
    let mut out = String::new();
    for stmt in &program.stmts {
        pp_stmt(stmt, 0, &mut out);
    }
    out
}

fn pp_indent(depth: usize, out: &mut String) {
    for _ in 0..depth {
        out.push_str("  ");
    }
}

fn pp_stmt(stmt: &Stmt, depth: usize, out: &mut String) {
    pp_indent(depth, out);
    match stmt {
        Stmt::Let { name, mutable, ty_annotation, expr, .. } => {
            out.push_str(&format!(
                "Let {}{}{} = ",
                if *mutable { "mut " } else { "" },
                name,
                ty_annotation
                    .as_ref()
                    .map(|t| format!(": {}", t.display()))
                    .unwrap_or_default()
            ));
            pp_expr(expr, out);
            out.push('\n');
        }
        Stmt::Assign { name, expr, .. } => {
            out.push_str(&format!("Assign {} = ", name));
            pp_expr(expr, out);
            out.push('\n');
        }
        Stmt::Expr(expr, _) => {
            pp_expr(expr, out);
            out.push('\n');
        }
        Stmt::Return(expr, _) => {
            out.push_str("Return ");
            pp_expr(expr, out);
            out.push('\n');
        }
        Stmt::FnDef { name, params, return_ty, body, .. } => {
            let params_str: Vec<String> = params
                .iter()
                .map(|p| format!("{}: {}", p.name, p.ty.display()))
                .collect();
            out.push_str(&format!(
                "fn {}({}) -> {} {{\n",
                name,
                params_str.join(", "),
                return_ty.display()
            ));
            for s in body {
                pp_stmt(s, depth + 1, out);
            }
            pp_indent(depth, out);
            out.push_str("}\n");
        }
        Stmt::Block(stmts, _) => {
            out.push_str("{\n");
            for s in stmts {
                pp_stmt(s, depth + 1, out);
            }
            pp_indent(depth, out);
            out.push_str("}\n");
        }
        Stmt::If { cond, then_branch, else_branch, .. } => {
            out.push_str("If ");
            pp_expr(cond, out);
            out.push_str(" {\n");
            for s in then_branch {
                pp_stmt(s, depth + 1, out);
            }
            pp_indent(depth, out);
            out.push('}');
            if let Some(else_stmts) = else_branch {
                out.push_str(" else {\n");
                for s in else_stmts {
                    pp_stmt(s, depth + 1, out);
                }
                pp_indent(depth, out);
                out.push('}');
            }
            out.push('\n');
        }
        Stmt::While { cond, body, .. } => {
            out.push_str("While ");
            pp_expr(cond, out);
            out.push_str(" {\n");
            for s in body {
                pp_stmt(s, depth + 1, out);
            }
            pp_indent(depth, out);
            out.push_str("}\n");
        }
    }
}

fn pp_expr(expr: &Expr, out: &mut String) {
    match expr {
        Expr::Literal(lit, _) => match lit {
            Literal::Number(n) => out.push_str(&format!("{}", n)),
            Literal::String(s) => out.push_str(&format!("\"{}\"", s)),
            Literal::Bool(b) => out.push_str(&format!("{}", b)),
        },
        Expr::Variable(name, _) => out.push_str(name),
        Expr::Binary { left, op, right, .. } => {
            out.push('(');
            pp_expr(left, out);
            out.push_str(&format!(" {} ", op.display()));
            pp_expr(right, out);
            out.push(')');
        }
        Expr::Unary { op, expr, .. } => {
            let sym = match op {
                UnaryOp::Not => "!",
                UnaryOp::Neg => "-",
            };
            out.push_str(sym);
            pp_expr(expr, out);
        }
        Expr::Call { callee, args, .. } => {
            pp_expr(callee, out);
            out.push('(');
            for (i, a) in args.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                pp_expr(a, out);
            }
            out.push(')');
        }
        Expr::ArrayLit { elements, .. } => {
            out.push('[');
            for (i, e) in elements.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                pp_expr(e, out);
            }
            out.push(']');
        }
        Expr::Index { array, index, .. } => {
            pp_expr(array, out);
            out.push('[');
            pp_expr(index, out);
            out.push(']');
        }
        Expr::Error(_) => out.push_str("<error>"),
    }
}
