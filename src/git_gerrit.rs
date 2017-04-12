use git::*;
use gerrit::*;

pub struct GitGerrit {
    pub git_info: GitInfo,
    pub gerrit: Gerrit,
}

impl GitGerrit {
    pub fn new () -> GitGerrit {
        GitGerrit {
            git_info: GitInfo::read(),
            gerrit: Gerrit::new()
        }
    }

    pub fn get_change (&self) -> Change {
        self.gerrit.change(&self.git_info.change_id())
    }
}
