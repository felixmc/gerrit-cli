use std::env;
use std::process::Command;
use std::io::Error;

mod cmd;
use cmd::Cmd;
use cmd::CmdMatch;
use cmd::CmdOption;
use cmd::CmdFactory;
use cmd::Arg;

mod open;
use open::OpenCmdFactory;


struct GerCmdFactory {}
impl CmdFactory<CmdMatch> for GetCmdFactory {
    fn create() -> CmdMatch {
        let open = Box::new(CmdOption {
            arg: Arg {
                names: vec!["open".to_owned()],
                info: "Open web pages related to the gerrit".to_owned(),
            },
            cmd: Box::new(OpenCmdFactory::create())
        });

        Box::new(CmdMatch {
            options: vec![open]
        })
    }
}


// fn print_usage(program: &str, opts: Options) {
//     let brief = format!("Usage: {} FILE [options]", program);
//     print!("{}", opts.usage(&brief));
// }



fn main () {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    println!("args: {:?}", &args[1..]);

    // let gerCmd = newGerCmd();

    // gerCmd.execute(&args[1..].to_vec())


    // let matches = match opts.parse(&args[1..]) {
    //     Ok(m) => { m }
    //     Err(f) => { panic!(f.to_string()) }
    // };
    //
    // // if matches.opt_present("h") {
    // //     print_usage(&program, opts);
    // //     return;
    // // }
    //
    // let output = matches.opt_str("o");
    // let input = if matches.free.is_empty() {
    //     print_usage(&program, opts);
    //     // return;
    //     "".to_owned()
    // } else {
    //     matches.free[0].clone()
    // };
    //
    // println!("input: {}", input);
    // println!("output: {:?}", output);
}
