use std::env;

mod exec;
mod git;
mod cmd;
mod open;

use cmd::*;
use open::*;

struct GerCmdFactory {}
impl CmdFactory for GerCmdFactory {
    fn create() -> Box<Cmd> {
        let open = Box::new(CmdOption {
            arg: Arg {
                names: vec!["open".to_owned()],
                info: "Open web pages related to the gerrit".to_owned(),
            },
            cmd: OpenCmd::create()
        });

        Box::new(CmdMatch {
            options: vec![open]
        })
    }
}

fn main () {
    let args: Vec<String> = env::args().collect();
    // let program = args[0].clone();

    let ger_cmd = GerCmdFactory::create();
    ger_cmd.execute(&args[1..]);
}
