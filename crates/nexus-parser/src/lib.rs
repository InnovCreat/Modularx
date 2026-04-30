pub mod ast;

// Re-export core AST types so consumers can `use nexus_parser::{Program, Stmt, Expr, ...}`
pub use ast::{
    Program, Stmt, Expr, Literal, Param,
    BinOp, UnaryOp, Ty,
    pretty_print,
};

use nexus_span::Span;
use nexus_token::{Token, TokenKind};

// ── Precedence levels for Pratt parsing ──────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Prec {
    Lowest,
    Or,       // ||
    And,      // &&
    Equality, // == !=
    Compare,  // < > <= >=
    Sum,      // + -
    Product,  // * /
    Prefix,   // -x  !x
    Call,     // f(x)
}

fn infix_prec(kind: &TokenKind) -> Option<Prec> {
    match kind {
        TokenKind::Or => Some(Prec::Or),
        TokenKind::And => Some(Prec::And),
        TokenKind::Eq | TokenKind::NotEq => Some(Prec::Equality),
        TokenKind::Lt | TokenKind::Gt | TokenKind::Le | TokenKind::Ge => Some(Prec::Compare),
        TokenKind::Plus | TokenKind::Minus => Some(Prec::Sum),
        TokenKind::Star | TokenKind::Slash => Some(Prec::Product),
        TokenKind::LParen => Some(Prec::Call),
        _ => None,
    }
}

fn token_to_binop(kind: &TokenKind) -> Option<BinOp> {
    match kind {
        TokenKind::Plus => Some(BinOp::Add),
        TokenKind::Minus => Some(BinOp::Sub),
        TokenKind::Star => Some(BinOp::Mul),
        TokenKind::Slash => Some(BinOp::Div),
        TokenKind::Eq => Some(BinOp::Eq),
        TokenKind::NotEq => Some(BinOp::NotEq),
        TokenKind::Lt => Some(BinOp::Lt),
        TokenKind::Gt => Some(BinOp::Gt),
        TokenKind::Le => Some(BinOp::Le),
        TokenKind::Ge => Some(BinOp::Ge),
        TokenKind::And => Some(BinOp::And),
        TokenKind::Or => Some(BinOp::Or),
        _ => None,
    }
}

// ── Parser ───────────────────────────────────────────────────

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    pub diagnostics: Vec<String>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            pos: 0,
            diagnostics: Vec::new(),
        }
    }

    pub fn parse(mut self) -> (Program, Vec<String>) {
        let mut stmts = Vec::new();
        while !self.at_eof() {
            stmts.push(self.parse_stmt());
        }
        (Program { stmts }, self.diagnostics)
    }

    // ── Helpers ──────────────────────────────────────────

    fn curr(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or_else(|| self.tokens.last().unwrap())
    }

    fn curr_kind(&self) -> &TokenKind {
        &self.curr().kind
    }

    fn curr_span(&self) -> Span {
        self.curr().span
    }

    fn at_eof(&self) -> bool {
        *self.curr_kind() == TokenKind::Eof
    }

    fn advance(&mut self) -> Token {
        let tok = self.curr().clone();
        if self.pos < self.tokens.len() - 1 {
            self.pos += 1;
        }
        tok
    }

    fn expect(&mut self, expected: TokenKind) -> Span {
        if *self.curr_kind() == expected {
            self.advance().span
        } else {
            self.diagnostics.push(format!(
                "Expected '{}', found '{}'",
                expected.display(),
                self.curr_kind().display()
            ));
            self.curr_span()
        }
    }

    // ── Statement parsing ────────────────────────────────

    fn parse_stmt(&mut self) -> Stmt {
        match self.curr_kind().clone() {
            TokenKind::Let => self.parse_let(),
            TokenKind::Fn => self.parse_fn_def(),
            TokenKind::Return => self.parse_return(),
            TokenKind::If => self.parse_if(),
            TokenKind::While => self.parse_while(),
            TokenKind::LBrace => self.parse_block_stmt(),
            TokenKind::Ident(_) => self.parse_ident_stmt(),
            _ => self.parse_expr_stmt(),
        }
    }

    fn parse_let(&mut self) -> Stmt {
        let start = self.advance().span; // consume 'let'
        let mutable = if *self.curr_kind() == TokenKind::Mut {
            self.advance();
            true
        } else {
            false
        };

        let name = match self.curr_kind().clone() {
            TokenKind::Ident(n) => {
                self.advance();
                n
            }
            _ => {
                self.diagnostics.push("Expected variable name after 'let'".into());
                self.advance();
                "<error>".into()
            }
        };

        // Optional type annotation: `: type`
        let ty_annotation = if *self.curr_kind() == TokenKind::Colon {
            self.advance();
            Some(self.parse_type())
        } else {
            None
        };

        self.expect(TokenKind::Assign);
        let expr = self.parse_expr(Prec::Lowest);
        let end = self.expect(TokenKind::Semicolon);

        Stmt::Let {
            name,
            mutable,
            ty_annotation,
            expr: Box::new(expr),
            span: start.union(end),
        }
    }

    fn parse_fn_def(&mut self) -> Stmt {
        let start = self.advance().span; // consume 'fn'

        let name = match self.curr_kind().clone() {
            TokenKind::Ident(n) => {
                self.advance();
                n
            }
            _ => {
                self.diagnostics.push("Expected function name".into());
                "<error>".into()
            }
        };

        self.expect(TokenKind::LParen);
        let params = self.parse_params();
        self.expect(TokenKind::RParen);

        // Optional return type: `-> type`
        let return_ty = if *self.curr_kind() == TokenKind::Arrow {
            self.advance();
            self.parse_type()
        } else {
            Ty::Unit
        };

        let (body, end) = self.parse_block();

        Stmt::FnDef {
            name,
            params,
            return_ty,
            body,
            span: start.union(end),
        }
    }

    fn parse_params(&mut self) -> Vec<Param> {
        let mut params = Vec::new();
        while *self.curr_kind() != TokenKind::RParen && !self.at_eof() {
            let span = self.curr_span();
            let name = match self.curr_kind().clone() {
                TokenKind::Ident(n) => {
                    self.advance();
                    n
                }
                _ => {
                    self.diagnostics.push("Expected parameter name".into());
                    self.advance();
                    "<error>".into()
                }
            };

            self.expect(TokenKind::Colon);
            let ty = self.parse_type();
            params.push(Param { name, ty, span });

            if *self.curr_kind() == TokenKind::Comma {
                self.advance();
            }
        }
        params
    }

    fn parse_type(&mut self) -> Ty {
        match self.curr_kind().clone() {
            TokenKind::TyI32 => {
                self.advance();
                Ty::I32
            }
            TokenKind::TyF64 => {
                self.advance();
                Ty::F64
            }
            TokenKind::TyBool => {
                self.advance();
                Ty::Bool
            }
            TokenKind::TyStr => {
                self.advance();
                Ty::Str
            }
            _ => {
                self.diagnostics.push(format!(
                    "Expected type, found '{}'",
                    self.curr_kind().display()
                ));
                Ty::Error
            }
        }
    }

    fn parse_return(&mut self) -> Stmt {
        let start = self.advance().span; // consume 'return'
        let expr = if *self.curr_kind() == TokenKind::Semicolon {
            Expr::Literal(Literal::Bool(true), start) // unit placeholder
        } else {
            self.parse_expr(Prec::Lowest)
        };
        let end = self.expect(TokenKind::Semicolon);
        Stmt::Return(Box::new(expr), start.union(end))
    }

    fn parse_if(&mut self) -> Stmt {
        let start = self.advance().span; // consume 'if'
        let cond = self.parse_expr(Prec::Lowest);
        let (then_branch, mut end) = self.parse_block();

        let else_branch = if *self.curr_kind() == TokenKind::Else {
            self.advance();
            let (stmts, e) = self.parse_block();
            end = e;
            Some(stmts)
        } else {
            None
        };

        Stmt::If {
            cond: Box::new(cond),
            then_branch,
            else_branch,
            span: start.union(end),
        }
    }

    fn parse_while(&mut self) -> Stmt {
        let start = self.advance().span; // consume 'while'
        let cond = self.parse_expr(Prec::Lowest);
        let (body, end) = self.parse_block();

        Stmt::While {
            cond: Box::new(cond),
            body,
            span: start.union(end),
        }
    }

    fn parse_block(&mut self) -> (Vec<Stmt>, Span) {
        let start = self.expect(TokenKind::LBrace);
        let mut stmts = Vec::new();
        while *self.curr_kind() != TokenKind::RBrace && !self.at_eof() {
            stmts.push(self.parse_stmt());
        }
        let end = self.expect(TokenKind::RBrace);
        (stmts, start.union(end))
    }

    fn parse_block_stmt(&mut self) -> Stmt {
        let (stmts, span) = self.parse_block();
        Stmt::Block(stmts, span)
    }

    fn parse_ident_stmt(&mut self) -> Stmt {
        let name = match self.curr_kind().clone() {
            TokenKind::Ident(n) => n,
            _ => unreachable!(),
        };
        let name_span = self.advance().span;

        // Assignment: `name = expr;`
        if *self.curr_kind() == TokenKind::Assign {
            self.advance();
            let expr = self.parse_expr(Prec::Lowest);
            let end = self.expect(TokenKind::Semicolon);
            return Stmt::Assign {
                name,
                expr: Box::new(expr),
                span: name_span.union(end),
            };
        }

        // Otherwise it's an expression statement starting with ident
        let expr = self.parse_expr_after_ident(name, name_span);
        let end = self.expect(TokenKind::Semicolon);
        Stmt::Expr(Box::new(expr), name_span.union(end))
    }

    fn parse_expr_stmt(&mut self) -> Stmt {
        let start = self.curr_span();
        let expr = self.parse_expr(Prec::Lowest);
        let end = self.expect(TokenKind::Semicolon);
        Stmt::Expr(Box::new(expr), start.union(end))
    }

    // ── Expression parsing (Pratt) ───────────────────────

    fn parse_expr(&mut self, min_prec: Prec) -> Expr {
        let mut left = self.parse_prefix();

        while let Some(prec) = infix_prec(self.curr_kind()) {
            if prec <= min_prec {
                break;
            }

            if *self.curr_kind() == TokenKind::LParen {
                // Function call
                left = self.parse_call(left);
            } else if let Some(op) = token_to_binop(self.curr_kind()) {
                self.advance();
                let right = self.parse_expr(prec);
                let span = left.span().union(right.span());
                left = Expr::Binary {
                    left: Box::new(left),
                    op,
                    right: Box::new(right),
                    span,
                };
            } else {
                break;
            }
        }

        left
    }

    fn parse_prefix(&mut self) -> Expr {
        match self.curr_kind().clone() {
            TokenKind::Number(n) => {
                let span = self.advance().span;
                Expr::Literal(Literal::Number(n), span)
            }
            TokenKind::StringLit(s) => {
                let span = self.advance().span;
                Expr::Literal(Literal::String(s), span)
            }
            TokenKind::True => {
                let span = self.advance().span;
                Expr::Literal(Literal::Bool(true), span)
            }
            TokenKind::False => {
                let span = self.advance().span;
                Expr::Literal(Literal::Bool(false), span)
            }
            TokenKind::Ident(name) => {
                let span = self.advance().span;
                Expr::Variable(name, span)
            }
            TokenKind::Minus => {
                let span = self.advance().span;
                let expr = self.parse_expr(Prec::Prefix);
                let full_span = span.union(expr.span());
                Expr::Unary {
                    op: UnaryOp::Neg,
                    expr: Box::new(expr),
                    span: full_span,
                }
            }
            TokenKind::Not => {
                let span = self.advance().span;
                let expr = self.parse_expr(Prec::Prefix);
                let full_span = span.union(expr.span());
                Expr::Unary {
                    op: UnaryOp::Not,
                    expr: Box::new(expr),
                    span: full_span,
                }
            }
            TokenKind::LParen => {
                self.advance();
                let expr = self.parse_expr(Prec::Lowest);
                self.expect(TokenKind::RParen);
                expr
            }
            _ => {
                let span = self.curr_span();
                self.diagnostics.push(format!(
                    "Unexpected token '{}' in expression",
                    self.curr_kind().display()
                ));
                self.advance();
                Expr::Error(span)
            }
        }
    }

    fn parse_call(&mut self, callee: Expr) -> Expr {
        self.advance(); // consume '('
        let mut args = Vec::new();
        while *self.curr_kind() != TokenKind::RParen && !self.at_eof() {
            args.push(self.parse_expr(Prec::Lowest));
            if *self.curr_kind() == TokenKind::Comma {
                self.advance();
            }
        }
        let end = self.expect(TokenKind::RParen);
        let span = callee.span().union(end);
        Expr::Call {
            callee: Box::new(callee),
            args,
            span,
        }
    }

    /// Parse expression continuation after we already consumed an identifier.
    fn parse_expr_after_ident(&mut self, name: String, name_span: Span) -> Expr {
        let mut left = if *self.curr_kind() == TokenKind::LParen {
            // Function call: `name(args...)`
            self.advance();
            let mut args = Vec::new();
            while *self.curr_kind() != TokenKind::RParen && !self.at_eof() {
                args.push(self.parse_expr(Prec::Lowest));
                if *self.curr_kind() == TokenKind::Comma {
                    self.advance();
                }
            }
            let end = self.expect(TokenKind::RParen);
            Expr::Call {
                callee: Box::new(Expr::Variable(name, name_span)),
                args,
                span: name_span.union(end),
            }
        } else {
            Expr::Variable(name, name_span)
        };

        // Continue with infix operators
        while let Some(prec) = infix_prec(self.curr_kind()) {
            if prec <= Prec::Lowest {
                break;
            }
            if *self.curr_kind() == TokenKind::LParen {
                left = self.parse_call(left);
            } else if let Some(op) = token_to_binop(self.curr_kind()) {
                self.advance();
                let right = self.parse_expr(prec);
                let span = left.span().union(right.span());
                left = Expr::Binary {
                    left: Box::new(left),
                    op,
                    right: Box::new(right),
                    span,
                };
            } else {
                break;
            }
        }

        left
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nexus_lexer::Lexer;

    fn parse_source(src: &str) -> (Program, Vec<String>) {
        let (tokens, _) = Lexer::new(src).tokenize();
        Parser::new(tokens).parse()
    }

    #[test]
    fn parse_let() {
        let (prog, diags) = parse_source("let x = 42;");
        assert!(diags.is_empty(), "diags: {:?}", diags);
        assert_eq!(prog.stmts.len(), 1);
        assert!(matches!(&prog.stmts[0], Stmt::Let { name, mutable: false, .. } if name == "x"));
    }

    #[test]
    fn parse_let_mut() {
        let (prog, diags) = parse_source("let mut y = 7;");
        assert!(diags.is_empty());
        assert!(matches!(&prog.stmts[0], Stmt::Let { mutable: true, .. }));
    }

    #[test]
    fn parse_fn_def() {
        let (prog, diags) = parse_source("fn add(a: i32, b: i32) -> i32 { return a + b; }");
        assert!(diags.is_empty(), "diags: {:?}", diags);
        assert!(matches!(&prog.stmts[0], Stmt::FnDef { name, params, .. } if name == "add" && params.len() == 2));
    }

    #[test]
    fn parse_if_else() {
        let (prog, diags) = parse_source("if x { let a = 1; } else { let b = 2; }");
        assert!(diags.is_empty(), "diags: {:?}", diags);
        assert!(matches!(&prog.stmts[0], Stmt::If { else_branch: Some(_), .. }));
    }

    #[test]
    fn parse_while() {
        let (prog, diags) = parse_source("while true { let a = 1; }");
        assert!(diags.is_empty(), "diags: {:?}", diags);
        assert!(matches!(&prog.stmts[0], Stmt::While { .. }));
    }

    #[test]
    fn parse_binary_ops() {
        let (prog, diags) = parse_source("let x = 1 + 2 * 3;");
        assert!(diags.is_empty(), "diags: {:?}", diags);
        // Should parse as 1 + (2 * 3) due to precedence
        if let Stmt::Let { expr, .. } = &prog.stmts[0] {
            assert!(matches!(**expr, Expr::Binary { op: BinOp::Add, .. }));
        } else {
            panic!("Expected let");
        }
    }

    #[test]
    fn parse_unary() {
        let (prog, diags) = parse_source("let x = -5;");
        assert!(diags.is_empty());
        if let Stmt::Let { expr, .. } = &prog.stmts[0] {
            assert!(matches!(**expr, Expr::Unary { op: UnaryOp::Neg, .. }));
        }
    }

    #[test]
    fn parse_call() {
        let (prog, diags) = parse_source("foo(1, 2);");
        assert!(diags.is_empty(), "diags: {:?}", diags);
        if let Stmt::Expr(expr, _) = &prog.stmts[0] {
            assert!(matches!(**expr, Expr::Call { .. }));
        }
    }
}
