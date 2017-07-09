use std::process::Command;
use std::process::Output;
use std::io::Error;
use std::str;
// use srd::marker::Copy;

pub type ExecResult = Result<ExecOutput, Error>;

pub struct ExecOutput {
    pub output: String,
}

impl ExecOutput {
    fn new (output: Output) -> ExecOutput {
        ExecOutput {
            output: ExecOutput::stdout_to_string(output),
        }
    }

    pub fn stdout_to_string (output: Output) -> String {
        output.stdout
            .clone()
            .into_iter()
            .map(|e| e as char)
            .collect()
    }

    // fn status (&self) {
    //     self.output.status
    // }
}

pub fn exec (cmd: &str, args: Vec<&str>) -> ExecResult {
    let mut command = Command::new(cmd);

    #[cfg(debug_assertions)]
    println!("EXEC: {0} {1}", cmd, args.join(" "));

    for arg in &args {
        command.arg(arg);
    }

    command.output()
        .map(|output| {
            let exec_output = ExecOutput::new(output);

            #[cfg(debug_assertions)]
            println!("{0}", exec_output.output);

            return exec_output
        })
}
