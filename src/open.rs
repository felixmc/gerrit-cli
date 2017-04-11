use exec::*;
use cmd::*;
use git::*;
use gerrit::*;

pub struct OpenCmd {}

impl OpenCmd {
    fn open_gerrit (&self, id: &str) {
        exec("open", vec![&format!("https://gerrit.instructure.com/{}", id)]);
    }

    fn open_diff (&self, id: &str) {
        exec("open", vec![&format!("https://gerrit.instructure.com/{}", id)]);
    }

    fn open_mobile (&self, id: &str) {
        exec("open", vec![&format!("https://gerrit.instructure.com/{}", id)]);
    }

    fn open_jira (&self, id: &str) {
        exec("open", vec![&format!("https://instructure.atlassian.net/browse/{}", id)]);
    }
}

impl Cmd for OpenCmd {
    fn get_help (&self) -> &str {
        ""
    }

    fn execute (&self, args: &[String]) {
        let git = GitInfo::read();

        if &args.len() > &0usize {
            match (&args.first()).unwrap().as_ref() {
                "jira" => self.open_jira(&git.jira_id()),
                "diff" => self.open_diff(&git.change_id()),
                "mobile" => self.open_mobile(&git.change_id()),
                _ => println!("{}", self.get_help())
            }
        } else {
            self.open_gerrit(&git.change_id())
        };
    }
}

impl OptionFactory for OpenCmd {
    fn option() -> Box<CmdOption> {
        Box::new(CmdOption {
            arg: Arg {
                names: vec!["open".to_owned()],
                info: "Open web pages related to the gerrit".to_owned(),
            },
            cmd: Box::new(OpenCmd {})
        })
    }
}
