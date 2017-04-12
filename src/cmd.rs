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
    fn execute(&self, &[String]);
}

pub struct FuncCmd {
    func: fn(&[String])
}

impl FuncCmd {
    pub fn option (names: Vec<&str>, info: &str, func: fn(&[String])) -> Box<CmdOption> {
        Box::new(CmdOption {
            arg: Arg {
                names: names.into_iter().map(|x| x.to_string()).collect(),
                info: info.to_owned(),
            },
            cmd: Box::new(FuncCmd { func: func })
        })
    }
}

impl Cmd for FuncCmd {
    fn execute (&self, args: &[String]) {
        (&self.func)(args)
    }
}

// combines an arg with a cmd
pub struct CmdOption {
    pub arg: Arg,
    pub cmd: Box<Cmd>,
}

// make CmdOption be treated as a command by proxying to inner command
impl Cmd for CmdOption {
    fn execute (&self, args: &[String]) {
        self.cmd.execute(args)
    }
}

// command that takes in command options and executes cmd based on args
pub struct CmdMatch {
    pub options: Vec<Box<CmdOption>>,
    pub default_behavior: fn(&CmdMatch)
}

impl CmdMatch {
    fn format_option (names: &str, info: &str) -> String {
        format!("\n  {:16}{}", names, info)
    }

    pub fn get_help(&self) -> String {
        self.options
            .iter()
            .fold(String::new(), |acc, opt| {
                let names = &opt.arg.names.iter().fold(String::new(), |names, name| format!("{} {}", name, names));
                let option_text = Self::format_option(names, &opt.arg.info);
                format!("{}{}", acc, option_text)
            }) + &Self::format_option("--help", "Display this help menu")
    }

    pub fn print_help (&self) {
        println!("Help:{}", self.get_help())
    }
}

impl Cmd for CmdMatch {
    fn execute(&self, args: &[String]) {
        match args.len() {
            0 => (self.default_behavior)(&self),
            _ => match &args[0][..] {
                "--help" => self.print_help(),
                _ => {
                    let opt = self.options.iter().find(|x| x.arg.matches(&args));
                    match opt {
                        Some(option) => option.cmd.execute(&args[1..]),
                        None => self.print_help(),
                    }
                }
            },
        }
    }
}

// thing that creates new options
pub trait OptionFactory {
    fn option() -> Box<CmdOption>;
}
