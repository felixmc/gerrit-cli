#[macro_use] extern crate serde_json;
#[macro_use] extern crate prettytable;
extern crate regex;
extern crate term;
extern crate ansi_term;

mod exec;
mod git;
mod gerrit;
mod git_gerrit;
mod print;
mod table;

mod cmd_open;
use cmd_open::*;

mod cmd_ls;
use cmd_ls::*;

mod cmd_status;
use cmd_status::*;

mod cmd;
use cmd::*;

// use ansi_term::*;

fn print_help (cmd: &CmdMatch) {
    cmd.print_help();
}

struct GerCmd {}
impl GerCmd {
    fn new() -> CmdMatch {
        CmdMatch {
            default_behavior: print_help,
            options: vec![
                OpenCmd::option(),
                LsCmd::option(),
                StatusCmd::option(),
            ]
        }
    }
}

fn main () {
    // std::panic::set_hook(Box::new(|x| {
    //     match x.payload().downcast_ref::<&str>() {
    //         // errors I expect like missing gerrit id in commit message, etc
    //         Some(panic) => {
    //             println!("Error: {}", panic);
    //             std::process::exit(1);
    //         }
    //         // something panic'd with a real runtime error
    //         None => {
    //             println!("OH NOES 😵\nsomething really 💥");
    //             std::process::exit(2);
    //         }
    //     }
    // }));

    let args: Vec<String> = std::env::args().collect();
    // let program = args[0].clone();

    let ger_cmd = GerCmd::new();
    ger_cmd.execute(&args[1..]);
}
