use prettytable;
use prettytable::cell::*;

use cmd::*;
use gerrit::*;
use print::*;

pub struct LsCmd {}
impl Cmd for LsCmd {
    fn execute(&self, _: &[String]) {
        let gerrit = Gerrit::new();
        let changes = gerrit.get_my_changes();
        let mut my_table = table!(["number", "subject", "project", "CR", "LR", "QA", "PR", "V"]);

        for change in changes {
            my_table.add_row(row![
                change.number,
                trim(&change.subject, 60),
                change.project,
                &review_to_cell(change.code_review),
                &review_to_cell(change.lint_review),
                &review_to_cell(change.qa_review),
                &review_to_cell(change.product_review),
                &review_to_cell(change.build_review)
            ]);
        }

        my_table.printstd();
    }
}

impl OptionFactory for LsCmd {
    fn option () -> Box<CmdOption> {
        Box::new(CmdOption {
            arg: Arg {
                names: vec!["ls".to_owned()],
                info: "list your open gerrit commits".to_owned(),
            },
            cmd: Box::new(LsCmd {})
        })
    }
}
