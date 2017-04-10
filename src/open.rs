use exec::*;
use cmd::*;
use git::*;

pub struct OpenCmd {}

impl OpenCmd {
    // it does stuff, private methods here?
    fn open_gerrit (&self, id: String) -> ExecResult {
        exec("open", vec![&format!("https://gerrit.instructure.com/{}", id)])
    }
}

impl Cmd for OpenCmd {
    fn get_help (&self) -> String {
        "".to_owned()
    }

    fn execute (&self, args: &[String]) {
        let git = GitInfo::read();

        match &args.first() {
            &Some(arg) => match arg.as_ref() {
                "jira" => (),
                "mobile" => (),
                _ => ()
            },
            &None => ()
        };
    }
}

impl CmdFactory for OpenCmd {
    fn create() -> Box<Cmd> {
        Box::new(OpenCmd {})
    }
}
