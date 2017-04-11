use std::env;
use std::io::Read;
use hyper::Client;
use json;
use json::JsonValue;

static gerrit_url: &'static str = "https://gerrit.instructure.com/";

pub struct Change {
    id: String,
    change_id: String,
}

pub struct Gerrit {
    user: String,
    pword: String,
}

impl Gerrit {
    pub fn new () -> Gerrit {
        Gerrit {
            user: env::var("GERRIT_USER").unwrap(),
            pword: env::var("GERRIT_PWORD").unwrap()
        }
    }

    fn get (path: &str) -> JsonValue {
        let url = format!("{}{}", gerrit_url, path);
        let client = Client::new();

        match client.get(&url).send() {
            Ok(mut res) => {
                let mut raw_json = String::new();
                res.read_to_string(&mut raw_json);
                println!("RAW_JSON: {}", raw_json);
                match json::parse(&raw_json) {
                    Ok(json_data) => json_data,
                    Err(error) => panic!("bad json from Gerrit")
                }
            },
            Err(error) => {
                panic!("network issues reaching gerrit ({}): {}", url, error)
            }
        }
    }

    pub fn change (change_id: &str) -> Change {
        let json_data = Self::get(&format!("changes/{}", change_id));
        Change {
            id: json_data["id"].as_str().unwrap().to_owned(),
            change_id: change_id.to_owned(),
        }
    }
}
