use viontin::{Command, Input, Output, ExitCode};

pub struct Greet;

impl Command for Greet {
    fn signature(&self) -> &str { "greet {name} {--shout} {--times=1}" }
    fn description(&self) -> &str { "Greet someone by name" }

    fn handle(&self, input: &Input, output: &Output) -> ExitCode {
        let name = match input.argument::<String>("name") {
            Ok(n) => n,
            Err(e) => { output.error(&e); return ExitCode::InvalidArgs; }
        };

        let times = input.option::<u64>("times").and_then(|r| r.ok()).unwrap_or(1);

        for _ in 0..times {
            if input.flag("shout") {
                output.success(&format!("HELLO, {}!", name.to_uppercase()));
            } else {
                output.success(&format!("Hello, {}!", name));
            }
        }

        ExitCode::Success
    }
}

pub struct Calc;

impl Command for Calc {
    fn signature(&self) -> &str { "calc {a} {b} {--op=add}" }
    fn description(&self) -> &str { "Perform a basic arithmetic operation" }

    fn handle(&self, input: &Input, output: &Output) -> ExitCode {
        let a = match input.argument::<i64>("a") {
            Ok(n) => n,
            Err(_) => { output.error("Invalid number: a"); return ExitCode::InvalidArgs; }
        };
        let b = match input.argument::<i64>("b") {
            Ok(n) => n,
            Err(_) => { output.error("Invalid number: b"); return ExitCode::InvalidArgs; }
        };

        let op = input.option::<String>("op").and_then(|r| r.ok()).unwrap_or_else(|| "add".into());

        let result = match op.as_str() {
            "add" | "+" => a + b,
            "sub" | "-" => a - b,
            "mul" | "*" => a * b,
            "div" | "/" => {
                if b == 0 { output.error("Division by zero"); return ExitCode::InvalidArgs; }
                a / b
            }
            _ => { output.error(&format!("Unknown operation: {}", op)); return ExitCode::InvalidArgs; }
        };

        output.success(&format!("{} {} {} = {}", a, op, b, result));
        ExitCode::Success
    }
}
