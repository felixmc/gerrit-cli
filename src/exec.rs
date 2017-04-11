use std::process::Command;
use std::process::Output;
use std::io::Error;
use std::str;

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
        let ref vec: Vec<u8> = self.output.stdout;
        let chars: Vec<char> = vec.iter().map(|&e| e as char).collect();
        chars.into_iter().collect()
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
