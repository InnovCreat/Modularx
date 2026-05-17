use nexus_span::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Literals
    Number(f64),
    StringLit(String),
    True,
    False,

    // Identifiers & keywords
    Ident(String),
    Let,
    Mut,
    Fn,
    Return,
    If,
    Else,
    While,

    // Type keywords
    TyI32,
    TyF64,
    TyBool,
    TyStr,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Assign,    // =
    Eq,        // ==
    NotEq,     // !=
    Lt,        // <
    Gt,        // >
    Le,        // <=
    Ge,        // >=
    Not,       // !
    And,       // &&
    Or,        // ||

    // Delimiters
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,  // [
    RBracket,  // ]
    Comma,
    Colon,
    Semicolon,
    Dot,       // .
    Arrow,     // ->

    // Special
    Eof,
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.display())
    }
}

impl TokenKind {
    /// Try to match an identifier string to a keyword.
    pub fn keyword(s: &str) -> Option<TokenKind> {
        match s {
            "let" => Some(TokenKind::Let),
            "mut" => Some(TokenKind::Mut),
            "fn" => Some(TokenKind::Fn),
            "return" => Some(TokenKind::Return),
            "if" => Some(TokenKind::If),
            "else" => Some(TokenKind::Else),
            "while" => Some(TokenKind::While),
            "true" => Some(TokenKind::True),
            "false" => Some(TokenKind::False),
            "i32" => Some(TokenKind::TyI32),
            "f64" => Some(TokenKind::TyF64),
            "bool" => Some(TokenKind::TyBool),
            "str" => Some(TokenKind::TyStr),
            _ => None,
        }
    }

    pub fn display(&self) -> &'static str {
        match self {
            TokenKind::Number(_) => "number",
            TokenKind::StringLit(_) => "string",
            TokenKind::True => "true",
            TokenKind::False => "false",
            TokenKind::Ident(_) => "identifier",
            TokenKind::Let => "let",
            TokenKind::Mut => "mut",
            TokenKind::Fn => "fn",
            TokenKind::Return => "return",
            TokenKind::If => "if",
            TokenKind::Else => "else",
            TokenKind::While => "while",
            TokenKind::TyI32 => "i32",
            TokenKind::TyF64 => "f64",
            TokenKind::TyBool => "bool",
            TokenKind::TyStr => "str",
            TokenKind::Plus => "+",
            TokenKind::Minus => "-",
            TokenKind::Star => "*",
            TokenKind::Slash => "/",
            TokenKind::Assign => "=",
            TokenKind::Eq => "==",
            TokenKind::NotEq => "!=",
            TokenKind::Lt => "<",
            TokenKind::Gt => ">",
            TokenKind::Le => "<=",
            TokenKind::Ge => ">=",
            TokenKind::Not => "!",
            TokenKind::And => "&&",
            TokenKind::Or => "||",
            TokenKind::LParen => "(",
            TokenKind::RParen => ")",
            TokenKind::LBrace => "{",
            TokenKind::RBrace => "}",
            TokenKind::LBracket => "[",
            TokenKind::RBracket => "]",
            TokenKind::Comma => ",",
            TokenKind::Colon => ":",
            TokenKind::Semicolon => ";",
            TokenKind::Dot => ".",
            TokenKind::Arrow => "->",
            TokenKind::Eof => "EOF",
        }
    }
}
