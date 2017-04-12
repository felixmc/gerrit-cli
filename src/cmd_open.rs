use exec::*;
use cmd::*;
use git::*;

fn open_gerrit (args: &[String]) {
    let id = match args.len() {
        0 => GitInfo::read().change_id(),
        _ => args[0].clone()
    };
    exec("open", vec![&format!("https://gerrit.instructure.com/r/{}", id)]).unwrap();
}

fn open_mobile (args: &[String]) {
    let id = match args.len() {
        0 => GitInfo::read().change_id(),
        _ => args[0].clone()
    };
    exec("open", vec![&format!("https://gerrit-mobile.inseng.net/#/c/{}", id)]).unwrap();
}

fn open_jira (args: &[String]) {
    let id = match args.len() {
        0 => GitInfo::read().jira_id(),
        _ => args[0].clone()
    };
    exec("open", vec![&format!("https://instructure.atlassian.net/browse/{}", id)]).unwrap();
}

fn default_behavior (_: &CmdMatch) {
    open_gerrit(&[])
}

pub struct OpenCmd {}
impl OpenCmd {
    fn command () -> Box<CmdMatch> {
        Box::new(CmdMatch {
            default_behavior: default_behavior,
            options: vec![
                FuncCmd::option(vec!["gerrit"], "open gerrit page of current commit", open_gerrit),
                FuncCmd::option(vec!["mobile"], "open gerrit mobile page of current commit", open_mobile),
                FuncCmd::option(vec!["jira"], "open jira ticket of current commit", open_jira),
            ]
        })
    }
}

impl OptionFactory for OpenCmd {
    fn option () -> Box<CmdOption> {
        Box::new(CmdOption {
            arg: Arg {
                names: vec!["open".to_owned()],
                info: "Open web pages related to the gerrit".to_owned(),
            },
            cmd: Self::command()
        })
    }
}
