use cmd::*;
use gerrit::*;
use git_gerrit::*;
use print::*;
use table::*;
use ansi_term::Colour::*;

pub struct StatusCmd {}
impl StatusCmd {}
impl Cmd for StatusCmd {
    fn execute(&self, _: &[String]) {
        let gerrit = GitGerrit::new();
        let change = gerrit.get_change();

        let (statusText, statusColor) = match (change.is_merged, change.can_merge, change.has_conflict) {
            (true, _, _) => ("✓ merged", Green),
            (_, true, _) => ("✓ ready", Cyan),
            (_, _, true) => ("✗ conflict", Red),
            _ => ("", Yellow)
        };

        // this is awfully hacky...but one of these days I'm going to abstract it into something pretty
        println!("+{}+", row_separator(78));
        println!("| {} |", cell_content_centered(change.subject, 76, White));

        println!("+{}+{}+{}+",  row_separator(15), row_separator(15), row_separator(46));
        println!("| {} | {} | {} |", cell_content_centered(statusText.to_owned(), 15, statusColor), cell_content_centered(change.number, 13, White), cell_content_centered(change.change_id, 44, White));
        println!("+{}+{}+{}+{}+", row_separator(15), row_separator(15), row_separator(14), row_separator(31));

        // robots
        println!("| {} | {} | {} |", cell_content("bots".to_owned(), 13, White), review_cell(change.build_review, 30), review_cell(change.lint_review, 29));
        println!("+{}+{}+{}+{}+{}+", row_separator(15), row_separator(20), row_separator(9), row_separator(10), row_separator(20));

        println!("| {} | {} | {} | {} |", cell_content("reviews".to_owned(), 13, White), review_cell(change.code_review, 20), review_cell(change.qa_review, 20), review_cell(change.product_review, 20));
        println!("+{}+{}+{}+{}+", row_separator(15), row_separator(20), row_separator(20), row_separator(20));
    }
}

impl OptionFactory for StatusCmd {
    fn option () -> Box<CmdOption> {
        Box::new(CmdOption {
            arg: Arg {
                names: vec!["status".to_owned()],
                info: "status of your current gerrit commit".to_owned(),
            },
            cmd: Box::new(StatusCmd {})
        })
    }
}
