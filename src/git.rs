use exec::*;
use regex::Regex;

pub struct GitInfo {
    data: String,
}

impl GitInfo {
    pub fn read () -> GitInfo {
        match exec("git", vec!["log", "-1"]) {
            Ok(result) => GitInfo { data: result.output },
            Err(err) => panic!("cannot read git data: {}", err)
        }
    }

    fn parse_value (&self, regex: Regex, group: usize) -> Option<String> {
        self.data.lines()
            .filter_map(|line| {
                regex.captures(line)
            })
            .nth(0)
            .map(|capture| {
                capture.get(group)
                    .map(|val| val.as_str().to_string())
                    .unwrap()
            })
    }

    pub fn change_id (&self) -> String {
        let regex = Regex::new(r"Change-Id: (I[a-f0-9]{40})$").unwrap();
        match self.parse_value(regex, 1) {
            Some(id) => id,
            None => panic!("no gerrit change id found in commit message")
        }
    }

    pub fn jira_id (&self) -> String {
        let regex = Regex::new(r"(refs|ref|fix|fixes|close|closes)\s+([A-Z]{2,5}-[0-9]{1,5})$").unwrap();
        match self.parse_value(regex, 2) {
            Some(id) => id,
            None => panic!("no jira tickets found in commit message")
        }
    }
}
