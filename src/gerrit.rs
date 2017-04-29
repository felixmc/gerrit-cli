use std::env;
// use std::io::Read;
// use ansi_term::Colour::{Red, Green};

use serde_json;
use serde_json::*;

use exec::*;

static gerrit_url: &'static str = "https://gerrit.instructure.com/";

pub enum ReviewResult {
    Rejected,
    Approved,
    Disliked,
    Liked,
    Neutral,
}

impl ReviewResult {
    pub fn value (&self) -> &str {
        match *self {
            ReviewResult::Rejected => "rejected",
            ReviewResult::Disliked => "disliked",
            ReviewResult::Liked => "liked",
            ReviewResult::Approved => "approved",
            ReviewResult::Neutral => "neutral",
        }
    }
}

pub struct ChangeReview {
    pub author: String,
    pub result: ReviewResult
}

impl ChangeReview {
    fn parse_json (json: &Value) -> Option<ChangeReview> {
        let (review, review_type) =  vec![
            ReviewResult::Rejected,
            ReviewResult::Disliked,
            ReviewResult::Liked,
            ReviewResult::Approved,
        ]
        .into_iter()
        .fold((None, ReviewResult::Neutral), |(found_review, prev_key), cur_key| match found_review {
            None => (json.get(cur_key.value()), cur_key),
            _ => (found_review, prev_key)
        });

        review.map(|json| ChangeReview {
            author: json["username"].as_str().unwrap().to_owned(),
            result: review_type,
        })
    }
}

pub struct ChangeStatus {
    pub project: String,
    pub subject: String,
    pub owner: String,
    pub number: String,
    pub change_id: String,
    pub insertions: usize,
    pub deletions: usize,
    pub code_review: Option<ChangeReview>,
    pub qa_review: Option<ChangeReview>,
    pub product_review: Option<ChangeReview>,
    pub lint_review: Option<ChangeReview>,
    pub build_review: Option<ChangeReview>,
    pub can_merge: bool,
    pub is_merged: bool,
    pub has_conflict: bool,
}

impl ChangeStatus {
    pub fn parse_json (json: &Value) -> ChangeStatus {
        ChangeStatus {
            project: json["project"].as_str().unwrap().to_owned(),
            subject: json["subject"].as_str().unwrap().to_owned(),
            owner: json["owner"]["username"].as_str().unwrap().to_owned(),
            number: json["_number"].as_u64().unwrap().to_string(),
            change_id: json["change_id"].as_str().unwrap().to_owned(),
            insertions: json["insertions"].as_u64().unwrap() as usize,
            deletions: json["deletions"].as_u64().unwrap() as usize,
            code_review: ChangeReview::parse_json(&json["labels"]["Non-Author-Review"]),
            qa_review: ChangeReview::parse_json(&json["labels"]["QA-Review"]),
            product_review: ChangeReview::parse_json(&json["labels"]["Code-Review"]),
            lint_review: ChangeReview::parse_json(&json["labels"]["Lint-Review"]),
            build_review: ChangeReview::parse_json(&json["labels"]["Verified"]),
            can_merge: json["submittable"].as_bool().unwrap_or(false),
            is_merged: json["status"].as_str().unwrap() == "MERGED", 
            has_conflict: !json["mergeable"].as_bool().unwrap_or(false),                       
        }
    }
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
            pword: match env::var("GERRIT_PWD") {
                Ok(val) => val,
                Err(_) => panic!("missing GERRIT_PWD in ENV")
            }
        }
    }

    fn get (&self, path: &str) -> Value {
        let url = format!("{}{}", gerrit_url, path);
        let user_pass = format!("{}:{}", self.user, self.pword);

        match exec("curl", vec!["--digest", "-u", &user_pass, &url]) {
            Ok(output) => {
                let str_output: Box<Vec<String>> = Box::new(output.stdout_to_string().lines().skip(1).map(|x| x.to_string()).collect());
                let raw_json = str_output.join("\n");
                match serde_json::from_str(&raw_json) {
                    Ok(json_data) => json_data,
                    Err(err) => panic!("bad json from gerrit: {}", err)
                }
            },
            Err(err) => panic!("cannot reach gerrit: {}", err)
        }
    }

    pub fn get_change (&self, change_id: &str) -> ChangeStatus {
        let json_data = self.get(&format!("a/changes/{}?o=LABELS&o=DETAILED_ACCOUNTS", change_id));
        ChangeStatus::parse_json(&json_data)
    }

    pub fn get_my_changes (&self) -> Vec<ChangeStatus> {
        let changes = self.get(&format!("a/changes/?q=owner:self+status:open&o=LABELS&o=DETAILED_ACCOUNTS"));
        changes.as_array().unwrap().iter().map(|change_json| {
            // println!("{:?}\n\n", change_json);
            ChangeStatus::parse_json(change_json)
        }).collect()
    }
}
