#[macro_use]
extern crate json;
extern crate hyper;
extern crate regex;

mod exec;
mod git;
mod gerrit;
mod cmd;
mod open;

use cmd::*;
use open::*;

struct GerCmd {}
impl GerCmd {
    fn new() -> Box<Cmd> {
        Box::new(CmdMatch {
            options: vec![
                OpenCmd::option()
            ]
        })
    }
}

fn main () {
    std::panic::set_hook(Box::new(|x| {
        println!("Error: {}", x.payload().downcast_ref::<&str>().unwrap());
        std::process::exit(1);
    }));

    let args: Vec<String> = std::env::args().collect();
    // let program = args[0].clone();

    let ger_cmd = GerCmd::new();
    ger_cmd.execute(&args[1..]);
}
