mod cmd;
use cmd::Cmd;
use cmd::CmdMatch;
use cmd::CmdOption;
use cmd::CmdFactory;
use cmd::Arg;

struct OpenCmd {
    // private data about opening
}

// impl OpenCmd {
//     // it does stuff, private methods here?
//     fn new() -> OpenCmd {
//         OpenCmd {}
//     }
// }

impl Cmd for OpenCmd {
    fn get_help (&self) -> String {
        "".to_owned()
    }

    fn execute (&self, args: &Vec<String>) {
        println!("EXECUTING OPEN")
    }
}

pub struct OpenCmdFactory {}

impl CmdFactory<OpenCmd> for OpenCmdFactory {
    fn create() -> {
        OpenCmd {}
    }
}
