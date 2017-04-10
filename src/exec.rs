use std::process::Command;
use std::process::Output;
use std::io::Error;

pub type ExecResult = Result<ExecOutput, Error>;

pub struct ExecOutput {
    output: Output
}

impl ExecOutput {
    fn new (output: Output) -> ExecOutput {
        ExecOutput {
            output: output
        }
    }

    pub fn stdout_to_string (&self) -> String {
        self.output.stdout.iter()
            .fold(String::new(), |sum, val| format!("{}{}", sum, *val as char))
    }

    // fn status (&self) {
    //     self.output.status
    // }
}

pub fn exec (cmd: &str, args: Vec<&str>) -> ExecResult {
    let mut command = Command::new(cmd);

    for arg in &args {
        command.arg(arg);
    }

    command.output()
        .map(|output| ExecOutput::new(output))
}
