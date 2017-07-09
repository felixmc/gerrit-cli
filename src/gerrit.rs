use std::env;
// use std::io::Read;
// use ansi_term::Colour::{Red, Green};

use serde_json;
use serde_json::*;

use curl;

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
    url: String,
}

impl Gerrit {
    pub fn new () -> Gerrit {
        Gerrit {
            user: env::var("GERRIT_USER").expect("Missing GERRIT_USER in env"),
            pword: env::var("GERRIT_PWD").expect("Missing GERRIT_PWD in env"),
            url: env::var("GERRIT_URL").expect("Missing GERRIT_URL in env"),
        }
    }

    fn get (&self, path: &str) -> Value {
        let url = format!("{}{}", &self.url, path);
        let user_pass = format!("{}:{}", self.user, self.pword);
        match curl::get(&url, vec!["-u", &user_pass]) {
            Ok(output) => match output.is_unauthorized() {
                true => panic!("Unauthorized response from gerrit. is your HTTP password up to date?"),
                false => match serde_json::from_str(&output.body_for_json()) {
                    Ok(json) => json,
                    Err(json_err) => {
                        #[cfg(debug_assertions)]
                        println!("URL: {0} \nJSON: {1}\n", path, output.body);

                        panic!("bad json from gerrit: {}", json_err)
                    }
                },
            },
            Err(curl_err) => panic!("cannot reach gerrit: {}", curl_err)
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
