use std::env;
use std::io::Read;

use json;
use json::JsonValue;

use exec::*;

static gerrit_url: &'static str = "https://gerrit.instructure.com/";

pub struct Change {
    pub number: String,
    pub change_id: String,
}

pub struct Gerrit {
    user: String,
    pword: String,
}

impl Gerrit {
    pub fn new () -> Gerrit {
        Gerrit {
            user: match env::var("GERRIT_USER") {
                Ok(val) => val,
                Err(_) => panic!("missing GERRIT_USER in ENV")
            },
            pword: match env::var("GERRIT_PWORD") {
                Ok(val) => val,
                Err(_) => panic!("missing GERRIT_PWORD in ENV")
            }
        }
    }

    fn get (&self, path: &str) -> JsonValue {
        let url = format!("{}{}", gerrit_url, path);
        let user_pass = format!("{}:{}", self.user, self.pword);

        match exec("curl", vec!["--digest", "-u", &user_pass, &url]) {
            Ok(output) => {
                let str_output: Box<Vec<String>> = Box::new(output.stdout_to_string().lines().skip(1).map(|x| x.to_string()).collect());
                let raw_json = str_output.join("\n");

                println!("RAW_JSON: {}", raw_json);
                match json::parse(&raw_json) {
                    Ok(json_data) => json_data,
                    Err(err) => panic!("bad json from gerrit: {}", err)
                }
            },
            Err(err) => panic!("cannot reach gerrit: {}", err)
        }
    }

    pub fn change (&self, change_id: &str) -> Change {
        let json_data = self.get(&format!("a/changes/{}", change_id));
        Change {
            number: json_data["_number"].as_str().unwrap().to_owned(),
            change_id: change_id.to_owned(),
        }
    }
}
