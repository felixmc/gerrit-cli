// defines <thing> that can tell us whether a series of arguments match
pub trait ArgMatch {
    fn matches(&self, &[String]) -> bool;
}

// we define an argument as something with a name and a description
pub struct Arg {
    pub names: Vec<String>,
    pub info: String,
}

// now we give any argument the ability to match its names by implementing ArgMatch
impl ArgMatch for Arg {
    fn matches(&self, args: &[String]) -> bool {
        match (&args).get(0) {
            Some(arg) => (&self.names).contains(&arg),
            None => false,
        }
    }
}

// define command <thing> that can be executed and has a help message
pub trait Cmd {
    fn get_help(&self) -> String;
    fn execute(&self, &[String]);
}

// combines an arg with a cmd
pub struct CmdOption {
    pub arg: Arg,
    pub cmd: Box<Cmd>,
}

// command that takes in command options and executes cmd based on args
pub struct CmdMatch {
    pub options: Vec<Box<CmdOption>>,
}

impl Cmd for CmdMatch {
    fn get_help(&self) -> String {
        "".to_owned() // TODO: real help
    }

    fn execute(&self, args: &[String]) {
        let opt = (&self.options).iter().find(|x| x.arg.matches(&args));
        match opt {
            Some(option) => option.cmd.execute(&args[1..]),
            None => println!("Help: {}", self.get_help()),
        }
    }
}

// thing that creates new cmd
pub trait CmdFactory {
    fn create() -> Box<Cmd>;
}
