/// A byte-offset span into the source text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

impl Span {
    pub fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    pub fn dummy() -> Self {
        Self { start: 0, end: 0 }
    }

    pub fn union(self, other: Span) -> Span {
        Span {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }

    pub fn len(self) -> u32 {
        self.end - self.start
    }

    pub fn is_empty(self) -> bool {
        self.start == self.end
    }
}

/// Maps byte offsets to (line, column) for diagnostics.
pub struct SourceMap {
    /// Byte offset of the start of each line.
    line_starts: Vec<u32>,
}

impl SourceMap {
    pub fn new(source: &str) -> Self {
        let mut line_starts = vec![0u32];
        for (i, ch) in source.char_indices() {
            if ch == '\n' {
                line_starts.push(i as u32 + 1);
            }
        }
        Self { line_starts }
    }

    /// Returns 1-based (line, column) for a byte offset.
    pub fn line_col(&self, offset: u32) -> (u32, u32) {
        let line = match self.line_starts.binary_search(&offset) {
            Ok(l) => l,
            Err(l) => l.saturating_sub(1),
        };
        let col = offset - self.line_starts[line];
        (line as u32 + 1, col + 1)
    }

    /// Extract the source text covered by a span.
    pub fn slice<'a>(&self, source: &'a str, span: Span) -> &'a str {
        &source[span.start as usize..span.end as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_col_basic() {
        let src = "let x = 42;\nlet y = 7;\n";
        let sm = SourceMap::new(src);
        assert_eq!(sm.line_col(0), (1, 1));   // 'l' of first let
        assert_eq!(sm.line_col(12), (2, 1));  // 'l' of second let
        assert_eq!(sm.line_col(16), (2, 5));  // 'y'
    }

    #[test]
    fn span_union() {
        let a = Span::new(5, 10);
        let b = Span::new(8, 20);
        let u = a.union(b);
        assert_eq!(u, Span::new(5, 20));
    }
}
