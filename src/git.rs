use exec::*;
use regex::Regex;

pub struct GitInfo {
    data: String
}

impl GitInfo {
    pub fn read () -> GitInfo {
        match exec("git", vec!["log", "-1"]) {
            Ok(output) => GitInfo { data: output.stdout_to_string() },
            Err(err) => panic!("Error reading git data: {}", err)
        }
    }

    fn parse_value (&self, regex: Regex) -> Option<String> {
        self.data.lines()
            .filter_map(|line| {
                regex.captures(line)
            })
            .nth(0)
            .map(|capture| {
                capture.get(1)
                    .map(|val| val.as_str().to_string())
                    .unwrap()
            })
    }

    pub fn change_id (&self) -> String {
        let regex = Regex::new(r"Change-Id: (I[a-f0-9]{40})$").unwrap();
        match self.parse_value(regex) {
            Some(id) => id,
            None => panic!("No Gerrit change-id found!")
        }
    }

    pub fn jira_id (&self) -> String {
        let regex = Regex::new(r"refs|ref|fix|fixes|close|closes.*([A-Z]{2-5}-[0-9]{1-5})$").unwrap();
        match self.parse_value(regex) {
            Some(id) => id,
            None => panic!("No Jira tickets found!")
        }
    }
}
