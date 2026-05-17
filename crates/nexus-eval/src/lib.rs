use std::collections::HashMap;
use nexus_parser::ast::*;

// ── Runtime values ───────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Array(Vec<Value>),
    Unit,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => {
                if n.fract() == 0.0 {
                    write!(f, "{}", *n as i64)
                } else {
                    write!(f, "{}", n)
                }
            }
            Value::String(s) => write!(f, "{}", s),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Array(elems) => {
                write!(f, "[")?;
                for (i, v) in elems.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            }
            Value::Unit => write!(f, "()"),
        }
    }
}

impl Value {
    pub fn display(&self) -> String {
        format!("{}", self)
    }
}

// Internal-only wrapper for early return propagation
#[derive(Debug, Clone, PartialEq)]
enum FlowValue {
    Val(Value),
    Return(Value),
}

impl FlowValue {
    fn into_value(self) -> Value {
        match self {
            FlowValue::Val(v) | FlowValue::Return(v) => v,
        }
    }

    fn is_return(&self) -> bool {
        matches!(self, FlowValue::Return(_))
    }
}

// ── Environment ──────────────────────────────────────────────

#[derive(Clone)]
struct FnDef {
    params: Vec<Param>,
    body: Vec<Stmt>,
}

#[derive(Clone)]
struct Env {
    vars: HashMap<String, Value>,
    functions: HashMap<String, FnDef>,
    output: Vec<String>,
}

impl Env {
    fn new() -> Self {
        Self {
            vars: HashMap::new(),
            functions: HashMap::new(),
            output: Vec::new(),
        }
    }
}

// ── Public API ───────────────────────────────────────────────

#[derive(Debug)]
pub struct EvalResult {
    pub value: Value,
    pub output: Vec<String>,
}

pub fn eval(program: &Program) -> Result<EvalResult, String> {
    let mut env = Env::new();
    let flow = eval_block(&program.stmts, &mut env)?;
    Ok(EvalResult {
        value: flow.into_value(),
        output: env.output,
    })
}

// ── Block evaluation ─────────────────────────────────────────

fn eval_block(stmts: &[Stmt], env: &mut Env) -> Result<FlowValue, String> {
    let mut result = FlowValue::Val(Value::Unit);
    for stmt in stmts {
        result = eval_stmt(stmt, env)?;
        if result.is_return() {
            return Ok(result);
        }
    }
    Ok(result)
}

// ── Statement evaluation ─────────────────────────────────────

fn eval_stmt(stmt: &Stmt, env: &mut Env) -> Result<FlowValue, String> {
    match stmt {
        Stmt::Let { name, expr, .. } => {
            let val = eval_expr(expr, env)?;
            env.vars.insert(name.clone(), val);
            Ok(FlowValue::Val(Value::Unit))
        }

        Stmt::Assign { name, expr, .. } => {
            let val = eval_expr(expr, env)?;
            if env.vars.contains_key(name) {
                env.vars.insert(name.clone(), val);
                Ok(FlowValue::Val(Value::Unit))
            } else {
                Err(format!("Undefined variable '{}'", name))
            }
        }

        Stmt::Expr(expr, _) => {
            eval_expr(expr, env)?;
            Ok(FlowValue::Val(Value::Unit))
        }

        Stmt::Return(expr, _) => {
            let val = eval_expr(expr, env)?;
            Ok(FlowValue::Return(val))
        }

        Stmt::FnDef { name, params, body, .. } => {
            env.functions.insert(name.clone(), FnDef {
                params: params.clone(),
                body: body.clone(),
            });
            Ok(FlowValue::Val(Value::Unit))
        }

        Stmt::Block(stmts, _) => eval_block(stmts, env),

        Stmt::If { cond, then_branch, else_branch, .. } => {
            let c = eval_expr(cond, env)?;
            match c {
                Value::Bool(true) => eval_block(then_branch, env),
                Value::Bool(false) => {
                    if let Some(else_stmts) = else_branch {
                        eval_block(else_stmts, env)
                    } else {
                        Ok(FlowValue::Val(Value::Unit))
                    }
                }
                _ => Err("If condition must be bool".into()),
            }
        }

        Stmt::While { cond, body, .. } => {
            let mut iterations = 0u64;
            loop {
                let c = eval_expr(cond, env)?;
                match c {
                    Value::Bool(true) => {
                        let result = eval_block(body, env)?;
                        if result.is_return() {
                            return Ok(result);
                        }
                    }
                    Value::Bool(false) => break,
                    _ => return Err("While condition must be bool".into()),
                }
                iterations += 1;
                if iterations > 1_000_000 {
                    return Err("Loop exceeded 1,000,000 iterations (infinite loop?)".into());
                }
            }
            Ok(FlowValue::Val(Value::Unit))
        }
    }
}

// ── Expression evaluation ────────────────────────────────────

fn eval_expr(expr: &Expr, env: &mut Env) -> Result<Value, String> {
    match expr {
        Expr::Literal(lit, _) => Ok(match lit {
            Literal::Number(n) => Value::Number(*n),
            Literal::String(s) => Value::String(s.clone()),
            Literal::Bool(b) => Value::Bool(*b),
        }),

        Expr::Variable(name, _) => {
            env.vars.get(name)
                .cloned()
                .ok_or_else(|| format!("Undefined variable '{}'", name))
        }

        Expr::Binary { left, op, right, .. } => {
            let l = eval_expr(left, env)?;
            let r = eval_expr(right, env)?;
            eval_binop(*op, l, r)
        }

        Expr::Unary { op, expr: inner, .. } => {
            let val = eval_expr(inner, env)?;
            match (op, val) {
                (UnaryOp::Neg, Value::Number(n)) => Ok(Value::Number(-n)),
                (UnaryOp::Not, Value::Bool(b)) => Ok(Value::Bool(!b)),
                (UnaryOp::Neg, _) => Err("'-' requires a number".into()),
                (UnaryOp::Not, _) => Err("'!' requires a bool".into()),
            }
        }

        Expr::Call { callee, args, .. } => {
            if let Expr::Variable(name, _) = &**callee {
                // Built-in: print(value)
                if name == "print" {
                    let mut parts = Vec::new();
                    for arg in args {
                        let val = eval_expr(arg, env)?;
                        parts.push(val.display());
                    }
                    let line = parts.join(" ");
                    env.output.push(line);
                    return Ok(Value::Unit);
                }

                // Built-in: len(array_or_string)
                if name == "len" {
                    if args.len() != 1 {
                        return Err(format!("'len' expects 1 arg, got {}", args.len()));
                    }
                    let val = eval_expr(&args[0], env)?;
                    return match val {
                        Value::Array(elems) => Ok(Value::Number(elems.len() as f64)),
                        Value::String(s) => Ok(Value::Number(s.len() as f64)),
                        _ => Err("'len' requires array or string".into()),
                    };
                }

                // Built-in: push(array_var_name, element)
                if name == "push" {
                    if args.len() != 2 {
                        return Err(format!("'push' expects 2 args, got {}", args.len()));
                    }
                    let elem = eval_expr(&args[1], env)?;
                    if let Expr::Variable(arr_name, _) = &args[0] {
                        if let Some(Value::Array(ref mut elems)) = env.vars.get_mut(arr_name) {
                            elems.push(elem);
                            return Ok(Value::Unit);
                        } else {
                            return Err(format!("'push' first arg must be an array variable, '{}' is not", arr_name));
                        }
                    } else {
                        return Err("'push' first arg must be a variable name".into());
                    }
                }

                if let Some(f) = env.functions.get(name).cloned() {
                    if args.len() != f.params.len() {
                        return Err(format!(
                            "'{}' expects {} args, got {}",
                            name, f.params.len(), args.len()
                        ));
                    }

                    // Evaluate args in caller's env
                    let mut arg_vals = Vec::new();
                    for arg in args {
                        arg_vals.push(eval_expr(arg, env)?);
                    }

                    // Create call environment with copies of functions and output
                    let mut call_env = Env {
                        vars: HashMap::new(),
                        functions: env.functions.clone(),
                        output: Vec::new(),
                    };

                    // Bind parameters
                    for (param, val) in f.params.iter().zip(arg_vals) {
                        call_env.vars.insert(param.name.clone(), val);
                    }

                    let result = eval_block(&f.body, &mut call_env)?;

                    // Merge output from the call
                    env.output.append(&mut call_env.output);

                    Ok(result.into_value())
                } else {
                    Err(format!("Undefined function '{}'", name))
                }
            } else {
                Err("Can only call named functions".into())
            }
        }

        Expr::ArrayLit { elements, .. } => {
            let mut elems = Vec::new();
            for e in elements {
                elems.push(eval_expr(e, env)?);
            }
            Ok(Value::Array(elems))
        }

        Expr::Index { array, index, .. } => {
            let arr_val = eval_expr(array, env)?;
            let idx_val = eval_expr(index, env)?;
            match (arr_val, idx_val) {
                (Value::Array(elems), Value::Number(n)) => {
                    let i = n as i64;
                    if i < 0 || i as usize >= elems.len() {
                        Err(format!("Index {} out of bounds (array length {})", i, elems.len()))
                    } else {
                        Ok(elems[i as usize].clone())
                    }
                }
                (Value::Array(_), _) => Err("Array index must be a number".into()),
                _ => Err("Cannot index into non-array value".into()),
            }
        }

        Expr::Error(_) => Err("Encountered error expression".into()),
    }
}

// ── Binary operator evaluation ───────────────────────────────

fn eval_binop(op: BinOp, left: Value, right: Value) -> Result<Value, String> {
    match (op, &left, &right) {
        // Arithmetic
        (BinOp::Add, Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
        (BinOp::Sub, Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
        (BinOp::Mul, Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
        (BinOp::Div, Value::Number(a), Value::Number(b)) => {
            if *b == 0.0 {
                Err("Division by zero".into())
            } else {
                // Integer truncation when both operands are whole numbers
                let result = a / b;
                if a.fract() == 0.0 && b.fract() == 0.0 {
                    Ok(Value::Number(result.trunc()))
                } else {
                    Ok(Value::Number(result))
                }
            }
        }

        // String concatenation
        (BinOp::Add, Value::String(a), Value::String(b)) => {
            Ok(Value::String(format!("{}{}", a, b)))
        }

        // Equality (any type)
        (BinOp::Eq, a, b) => Ok(Value::Bool(a == b)),
        (BinOp::NotEq, a, b) => Ok(Value::Bool(a != b)),

        // Comparison (numbers only)
        (BinOp::Lt, Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a < b)),
        (BinOp::Gt, Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a > b)),
        (BinOp::Le, Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a <= b)),
        (BinOp::Ge, Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a >= b)),

        // Boolean logic
        (BinOp::And, Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(*a && *b)),
        (BinOp::Or, Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(*a || *b)),

        _ => Err(format!(
            "Cannot apply '{}' to {} and {}",
            op.display(), left.display(), right.display()
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nexus_lexer::Lexer;
    use nexus_parser::Parser;

    fn run(src: &str) -> Result<EvalResult, String> {
        let (tokens, _) = Lexer::new(src).tokenize();
        let (program, _) = Parser::new(tokens).parse();
        eval(&program)
    }

    fn run_val(src: &str) -> Value {
        run(src).expect("eval failed").value
    }

    #[test]
    fn arithmetic() {
        assert_eq!(run_val("let x = 2 + 3 * 4;"), Value::Unit);
    }

    #[test]
    fn variable() {
        let r = run("let x = 42; print(x);").unwrap();
        assert_eq!(r.output, vec!["42"]);
    }

    #[test]
    fn function_call() {
        let r = run("fn double(n: i32) -> i32 { return n * 2; } print(double(21));").unwrap();
        assert_eq!(r.output, vec!["42"]);
    }

    #[test]
    fn factorial() {
        let src = r#"
            fn factorial(n: i32) -> i32 {
                if n <= 1 { return 1; }
                return n * factorial(n - 1);
            }
            print(factorial(5));
        "#;
        let r = run(src).unwrap();
        assert_eq!(r.output, vec!["120"]);
    }

    #[test]
    fn fibonacci() {
        let src = r#"
            fn fib(n: i32) -> i32 {
                if n <= 1 { return n; }
                return fib(n - 1) + fib(n - 2);
            }
            print(fib(10));
        "#;
        let r = run(src).unwrap();
        assert_eq!(r.output, vec!["55"]);
    }

    #[test]
    fn while_loop() {
        let src = r#"
            let mut x = 0;
            let mut sum = 0;
            while x < 5 {
                x = x + 1;
                sum = sum + x;
            }
            print(sum);
        "#;
        let r = run(src).unwrap();
        assert_eq!(r.output, vec!["15"]);
    }

    #[test]
    fn if_else() {
        let src = r#"
            let x = 10;
            if x > 5 {
                print("big");
            } else {
                print("small");
            }
        "#;
        let r = run(src).unwrap();
        assert_eq!(r.output, vec!["big"]);
    }

    #[test]
    fn string_concat() {
        let src = r#"
            let a = "hello";
            let b = " world";
            print(a + b);
        "#;
        let r = run(src).unwrap();
        assert_eq!(r.output, vec!["hello world"]);
    }

    #[test]
    fn boolean_logic() {
        let src = r#"
            let x = true && false;
            let y = true || false;
            print(x);
            print(y);
        "#;
        let r = run(src).unwrap();
        assert_eq!(r.output, vec!["false", "true"]);
    }

    #[test]
    fn unary_ops() {
        let src = r#"
            print(-42);
            print(!true);
        "#;
        let r = run(src).unwrap();
        assert_eq!(r.output, vec!["-42", "false"]);
    }

    #[test]
    fn division_by_zero() {
        let r = run("let x = 1 / 0;");
        assert!(r.is_err());
        assert!(r.unwrap_err().contains("Division by zero"));
    }

    #[test]
    fn gcd() {
        // Since NEXUS uses f64 for all numbers, a/b is float division.
        // We implement modulo as: a - floor(a/b)*b using integer truncation.
        // gcd(48, 18): 48 / 18 = 2.666.. -> truncated via i32 cast trick
        // Simpler approach: iterative subtraction-based GCD
        let src = r#"
            fn gcd(a: i32, b: i32) -> i32 {
                let mut x = a;
                let mut y = b;
                while y != 0 {
                    let t = y;
                    y = x - (x / y) * y;
                    x = t;
                }
                return x;
            }
            print(gcd(48, 18));
        "#;
        let r = run(src).unwrap();
        assert_eq!(r.output, vec!["6"]);
    }

    #[test]
    fn array_literal_and_index() {
        let src = r#"
            let arr = [10, 20, 30];
            print(arr[0]);
            print(arr[2]);
        "#;
        let r = run(src).unwrap();
        assert_eq!(r.output, vec!["10", "30"]);
    }

    #[test]
    fn array_len() {
        let src = r#"
            let arr = [1, 2, 3, 4, 5];
            print(len(arr));
        "#;
        let r = run(src).unwrap();
        assert_eq!(r.output, vec!["5"]);
    }

    #[test]
    fn array_push() {
        let src = r#"
            let mut arr = [1, 2];
            push(arr, 3);
            print(len(arr));
            print(arr[2]);
        "#;
        let r = run(src).unwrap();
        assert_eq!(r.output, vec!["3", "3"]);
    }

    #[test]
    fn array_index_out_of_bounds() {
        let r = run("let arr = [1, 2]; let x = arr[5];");
        assert!(r.is_err());
        assert!(r.unwrap_err().contains("out of bounds"));
    }
}
