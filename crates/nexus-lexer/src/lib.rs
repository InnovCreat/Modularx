use nexus_span::Span;
use nexus_token::{Token, TokenKind};

pub struct Lexer<'a> {
    src: &'a [u8],
    pos: u32,
    diagnostics: Vec<String>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            src: source.as_bytes(),
            pos: 0,
            diagnostics: Vec::new(),
        }
    }

    pub fn tokenize(mut self) -> (Vec<Token>, Vec<String>) {
        let mut tokens = Vec::new();
        loop {
            let tok = self.next_token();
            let is_eof = tok.kind == TokenKind::Eof;
            tokens.push(tok);
            if is_eof {
                break;
            }
        }
        (tokens, self.diagnostics)
    }

    fn peek(&self) -> u8 {
        if (self.pos as usize) < self.src.len() {
            self.src[self.pos as usize]
        } else {
            0
        }
    }

    fn peek_next(&self) -> u8 {
        let i = self.pos as usize + 1;
        if i < self.src.len() {
            self.src[i]
        } else {
            0
        }
    }

    fn advance(&mut self) -> u8 {
        let ch = self.peek();
        self.pos += 1;
        ch
    }

    fn at_end(&self) -> bool {
        self.pos as usize >= self.src.len()
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            // whitespace
            while !self.at_end() && self.peek().is_ascii_whitespace() {
                self.advance();
            }
            // line comments
            if self.peek() == b'/' && self.peek_next() == b'/' {
                while !self.at_end() && self.peek() != b'\n' {
                    self.advance();
                }
                continue;
            }
            break;
        }
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace_and_comments();

        if self.at_end() {
            return Token {
                kind: TokenKind::Eof,
                span: Span::new(self.pos, self.pos),
            };
        }

        let start = self.pos;
        let ch = self.advance();

        let kind = match ch {
            b'+' => TokenKind::Plus,
            b'*' => TokenKind::Star,
            b'/' => TokenKind::Slash,
            b'(' => TokenKind::LParen,
            b')' => TokenKind::RParen,
            b'{' => TokenKind::LBrace,
            b'}' => TokenKind::RBrace,
            b',' => TokenKind::Comma,
            b':' => TokenKind::Colon,
            b';' => TokenKind::Semicolon,

            b'-' => {
                if self.peek() == b'>' {
                    self.advance();
                    TokenKind::Arrow
                } else {
                    TokenKind::Minus
                }
            }

            b'=' => {
                if self.peek() == b'=' {
                    self.advance();
                    TokenKind::Eq
                } else {
                    TokenKind::Assign
                }
            }

            b'!' => {
                if self.peek() == b'=' {
                    self.advance();
                    TokenKind::NotEq
                } else {
                    TokenKind::Not
                }
            }

            b'<' => {
                if self.peek() == b'=' {
                    self.advance();
                    TokenKind::Le
                } else {
                    TokenKind::Lt
                }
            }

            b'>' => {
                if self.peek() == b'=' {
                    self.advance();
                    TokenKind::Ge
                } else {
                    TokenKind::Gt
                }
            }

            b'&' => {
                if self.peek() == b'&' {
                    self.advance();
                    TokenKind::And
                } else {
                    self.diagnostics
                        .push(format!("Unexpected '&' at offset {}", start));
                    return self.next_token();
                }
            }

            b'|' => {
                if self.peek() == b'|' {
                    self.advance();
                    TokenKind::Or
                } else {
                    self.diagnostics
                        .push(format!("Unexpected '|' at offset {}", start));
                    return self.next_token();
                }
            }

            b'"' => return self.lex_string(start),

            c if c.is_ascii_digit() => return self.lex_number(start),

            c if c.is_ascii_alphabetic() || c == b'_' => return self.lex_ident(start),

            other => {
                self.diagnostics
                    .push(format!("Unexpected character '{}' at offset {}", other as char, start));
                return self.next_token();
            }
        };

        Token {
            kind,
            span: Span::new(start, self.pos),
        }
    }

    fn lex_number(&mut self, start: u32) -> Token {
        while !self.at_end() && self.peek().is_ascii_digit() {
            self.advance();
        }
        // fractional part
        if self.peek() == b'.' && self.peek_next().is_ascii_digit() {
            self.advance(); // consume '.'
            while !self.at_end() && self.peek().is_ascii_digit() {
                self.advance();
            }
        }
        let text = std::str::from_utf8(&self.src[start as usize..self.pos as usize]).unwrap();
        let value: f64 = text.parse().unwrap_or(0.0);
        Token {
            kind: TokenKind::Number(value),
            span: Span::new(start, self.pos),
        }
    }

    fn lex_string(&mut self, start: u32) -> Token {
        let mut value = String::new();
        while !self.at_end() && self.peek() != b'"' {
            let ch = self.advance();
            if ch == b'\\' && !self.at_end() {
                let escaped = self.advance();
                match escaped {
                    b'n' => value.push('\n'),
                    b't' => value.push('\t'),
                    b'\\' => value.push('\\'),
                    b'"' => value.push('"'),
                    _ => {
                        value.push('\\');
                        value.push(escaped as char);
                    }
                }
            } else {
                value.push(ch as char);
            }
        }
        if self.at_end() {
            self.diagnostics
                .push(format!("Unterminated string starting at offset {}", start));
        } else {
            self.advance(); // consume closing '"'
        }
        Token {
            kind: TokenKind::StringLit(value),
            span: Span::new(start, self.pos),
        }
    }

    fn lex_ident(&mut self, start: u32) -> Token {
        while !self.at_end() && (self.peek().is_ascii_alphanumeric() || self.peek() == b'_') {
            self.advance();
        }
        let text = std::str::from_utf8(&self.src[start as usize..self.pos as usize]).unwrap();
        let kind = TokenKind::keyword(text).unwrap_or_else(|| TokenKind::Ident(text.to_string()));
        Token {
            kind,
            span: Span::new(start, self.pos),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tokens() {
        let (tokens, diags) = Lexer::new("let x = 42;").tokenize();
        assert!(diags.is_empty());
        assert_eq!(tokens.len(), 5 + 1); // let x = 42 ; EOF
        assert_eq!(tokens[0].kind, TokenKind::Let);
        assert!(matches!(&tokens[1].kind, TokenKind::Ident(s) if s == "x"));
        assert_eq!(tokens[2].kind, TokenKind::Assign);
        assert!(matches!(tokens[3].kind, TokenKind::Number(n) if n == 42.0));
        assert_eq!(tokens[4].kind, TokenKind::Semicolon);
    }

    #[test]
    fn operators() {
        let (tokens, _) = Lexer::new("== != <= >= && || -> !").tokenize();
        let kinds: Vec<_> = tokens.iter().map(|t| &t.kind).collect();
        assert_eq!(*kinds[0], TokenKind::Eq);
        assert_eq!(*kinds[1], TokenKind::NotEq);
        assert_eq!(*kinds[2], TokenKind::Le);
        assert_eq!(*kinds[3], TokenKind::Ge);
        assert_eq!(*kinds[4], TokenKind::And);
        assert_eq!(*kinds[5], TokenKind::Or);
        assert_eq!(*kinds[6], TokenKind::Arrow);
        assert_eq!(*kinds[7], TokenKind::Not);
    }

    #[test]
    fn string_literal() {
        let (tokens, _) = Lexer::new(r#""hello\nworld""#).tokenize();
        assert!(matches!(&tokens[0].kind, TokenKind::StringLit(s) if s == "hello\nworld"));
    }

    #[test]
    fn comments_skipped() {
        let (tokens, _) = Lexer::new("42 // comment\n7").tokenize();
        assert!(matches!(tokens[0].kind, TokenKind::Number(n) if n == 42.0));
        assert!(matches!(tokens[1].kind, TokenKind::Number(n) if n == 7.0));
    }
}
