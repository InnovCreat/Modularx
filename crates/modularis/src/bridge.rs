// ════════════════════════════════════════════════════════════════
// BRIDGE — Real NEXUS compiler wired into the orchestrator
// Text signals containing source code get compiled through the
// full pipeline: Lex → Parse → TypeCheck → Eval
// ════════════════════════════════════════════════════════════════

use crate::{Channel, Signal, Subsystem, Voice};

use nexus_lexer::Lexer;
use nexus_parser::Parser;
use nexus_parser::ast::pretty_print;
use nexus_typeck::type_check;
use nexus_eval::{eval, Value};

// ── Compilation Result ──────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct CompileResult {
    pub source: String,
    pub token_count: usize,
    pub ast_text: String,
    pub type_errors: Vec<String>,
    pub output: Vec<String>,
    pub value: Option<String>,
    pub error: Option<String>,
    pub pulse: u64,
}

// ── NEXUS Compiler Subsystem ────────────────────────────────────
// Receives Text signals, compiles them, stores results.

pub struct NexusCompiler {
    sources: Vec<String>,
    results: Vec<CompileResult>,
    last_pulse: u64,
}

impl NexusCompiler {
    pub fn new() -> Self {
        Self {
            sources: Vec::new(),
            results: Vec::new(),
            last_pulse: 0,
        }
    }

    pub fn submit(&mut self, source: &str, pulse: u64) -> CompileResult {
        self.sources.push(source.to_string());

        let (tokens, _lex_diags) = Lexer::new(source).tokenize();
        let token_count = tokens.len().saturating_sub(1);

        let (program, _parse_diags) = Parser::new(tokens).parse();
        let ast_text = pretty_print(&program);

        let type_errors = type_check(&program);

        let (output, value, error) = if type_errors.is_empty() {
            match eval(&program) {
                Ok(result) => {
                    let val_str = match &result.value {
                        Value::Unit => None,
                        v => Some(v.display()),
                    };
                    (result.output, val_str, None)
                }
                Err(e) => (Vec::new(), None, Some(e)),
            }
        } else {
            (Vec::new(), None, None)
        };

        let result = CompileResult {
            source: source.to_string(),
            token_count,
            ast_text,
            type_errors,
            output,
            value,
            error,
            pulse,
        };

        self.results.push(result.clone());
        result
    }

    pub fn results(&self) -> &[CompileResult] {
        &self.results
    }

    pub fn last_result(&self) -> Option<&CompileResult> {
        self.results.last()
    }

    pub fn compile_count(&self) -> usize {
        self.results.len()
    }
}

impl Default for NexusCompiler {
    fn default() -> Self {
        Self::new()
    }
}

impl Subsystem for NexusCompiler {
    fn name(&self) -> &str {
        "NEXUS-COMPILER"
    }

    fn accepts(&self) -> &[Channel] {
        &[Channel::Text]
    }

    fn process(&mut self, voice: &Voice) -> Option<Signal> {
        if let Signal::Text { token_count, checksum } = voice.signal {
            // Use checksum as a source identifier — real integration would
            // carry the actual source string via a side channel.
            // For now, record that we received a compilable signal.
            self.last_pulse = voice.mark.pulse;

            // Emit a result signal with the processed token count
            Some(Signal::Text {
                token_count,
                checksum,
            })
        } else {
            None
        }
    }

    fn tick(&mut self, pulse: u64) {
        self.last_pulse = pulse;
    }
}

// ── Tests ───────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compile_simple() {
        let mut compiler = NexusCompiler::new();
        let result = compiler.submit("let x = 42; print(x);", 0);
        assert_eq!(result.token_count, 10);
        assert!(result.type_errors.is_empty());
        assert_eq!(result.output, vec!["42"]);
        assert!(result.error.is_none());
    }

    #[test]
    fn compile_with_arrays() {
        let mut compiler = NexusCompiler::new();
        let result = compiler.submit("let arr = [1, 2, 3]; print(arr[1]); print(len(arr));", 10);
        assert!(result.type_errors.is_empty());
        assert_eq!(result.output, vec!["2", "3"]);
    }

    #[test]
    fn compile_function() {
        let mut compiler = NexusCompiler::new();
        let src = "fn double(n: i32) -> i32 { return n * 2; } print(double(21));";
        let result = compiler.submit(src, 639);
        assert!(result.type_errors.is_empty());
        assert_eq!(result.output, vec!["42"]);
        assert_eq!(result.pulse, 639);
    }

    #[test]
    fn compile_type_error() {
        let mut compiler = NexusCompiler::new();
        let result = compiler.submit("let x = 1; x = 2;", 0);
        assert!(!result.type_errors.is_empty());
    }

    #[test]
    fn compile_runtime_error() {
        let mut compiler = NexusCompiler::new();
        let result = compiler.submit("let x = 1 / 0;", 0);
        assert!(result.error.is_some());
        assert!(result.error.unwrap().contains("Division by zero"));
    }

    #[test]
    fn multiple_compilations() {
        let mut compiler = NexusCompiler::new();
        compiler.submit("print(1);", 0);
        compiler.submit("print(2);", 1);
        compiler.submit("print(3);", 2);
        assert_eq!(compiler.compile_count(), 3);
        assert_eq!(compiler.last_result().unwrap().output, vec!["3"]);
    }

    #[test]
    fn subsystem_trait() {
        let compiler = NexusCompiler::new();
        assert_eq!(compiler.name(), "NEXUS-COMPILER");
        assert_eq!(compiler.accepts(), &[Channel::Text]);
    }
}
