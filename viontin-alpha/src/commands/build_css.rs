use viontin::{Command, Input, Output, ExitCode};

pub struct BuildCss;

impl Command for BuildCss {
    fn signature(&self) -> &str { "build:css" }
    fn description(&self) -> &str { "Compile TailwindCSS classes into a CSS bundle" }

    fn handle(&self, _input: &Input, output: &Output) -> ExitCode {
        let project_root = match std::env::current_dir() {
            Ok(d) => d,
            Err(e) => { output.error(&e.to_string()); return ExitCode::Failure; }
        };

        match viontin_gem_tailwind::compile_project(&project_root) {
            Ok(css) => {
                let line_count = css.lines().count();
                let output_path = project_root.join("assets").join("tailwind.css");
                output.success(&format!("CSS generated: {} ({} lines)", output_path.display(), line_count));
                ExitCode::Success
            }
            Err(e) => {
                output.error(&format!("Tailwind compilation failed: {}", e));
                ExitCode::Failure
            }
        }
    }
}
