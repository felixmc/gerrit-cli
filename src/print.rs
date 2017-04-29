// use std::str::FromStr;

use term::{Attr, color};
use prettytable;
use prettytable::cell::Cell;
use ansi_term::Colour::{Red, Green};

use gerrit::{ChangeReview, ReviewResult};

pub fn review_to_cell (review_opt: Option<ChangeReview>) -> String {
    match review_opt {
        Some(review) => match review.result {
	        ReviewResult::Rejected => "✗".to_string(),
	        ReviewResult::Disliked => "-1".to_string(),
	        ReviewResult::Liked => "+1".to_string(),
	        ReviewResult::Approved => "✓".to_string(),
	        _ => " ".to_string(),
	    },
        None => " ".to_string()
    }
}

pub fn trim (input: &String, len: usize) -> &str {
    if input.len() > len {
        &input[..len]
    } else {
        &input[..]
    }
}
