use viontin::{Command, Input, Output, ExitCode};

pub struct Greet;

impl Command for Greet {
    fn signature(&self) -> &str { "greet {name} {--shout}" }
    fn description(&self) -> &str { "Greet someone by name" }

    fn handle(&self, input: &Input, output: &Output) -> ExitCode {
        let name = match input.argument::<String>("name") {
            Ok(n) => n,
            Err(e) => { output.error(&e); return ExitCode::InvalidArgs; }
        };

        if input.flag("shout") {
            output.success(&format!("HELLO, {}!", name.to_uppercase()));
        } else {
            output.success(&format!("Hello, {}!", name));
        }

        ExitCode::Success
    }
}
