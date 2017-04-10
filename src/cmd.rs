// extern crate getopts;

// defines <thing> that can tell us whether a series of arguments match
pub trait ArgMatch {
    fn matches(&self, &Vec<String>) -> bool;
}

// we define an argument as something with a name and a description
pub struct Arg {
    pub names: Vec<String>,
    pub info: String,
}

// now we give any argument the ability to match its names by implementing ArgMatch
impl ArgMatch for Arg {
    fn matches(&self, args: &Vec<String>) -> bool {
        match (&args).get(0) {
            Some(arg) => (&self.names).contains(&arg),
            None => false,
        }
    }
}


// pub struct FlagArg {
//     arg: Arg,
// }
//
// impl FlagArg {}

pub trait Cmd {
    fn get_help(&self) -> String;
    fn execute(&self, &Vec<String>);
}

pub struct CmdOption {
    pub arg: Arg,
    pub cmd: Box<Cmd>,
}

pub struct CmdMatch {
    pub options: Vec<Box<CmdOption>>,
}

// impl Cmd for CmdMatch {
//     fn execute(&self, args: Vec<String>) {}
// }

impl Cmd for CmdMatch {
    fn get_help(&self) -> String {
        "".to_owned()
    }

    fn execute(&self, args: &Vec<String>) {
        let opt = (&self.options).iter().find(|x| x.arg.matches(&args));
        match opt {
            Some(option) => option.cmd.execute(args),
            None => println!("Help: {}", self.get_help()),
        }
    }
}

pub trait CmdFactory<T: Cmd> {
    fn create() -> T;
}
