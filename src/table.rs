use ansi_term::Colour;
use ansi_term::Colour::*;
use gerrit::*;

pub fn row_separator (span: usize) -> String {
	"-".repeat(span)
}

pub fn cell_content (data: String, span: usize, color: Colour) -> String {
	let str_len = data.len();
	let spacing = span - str_len;
	format!("{}{}", color.paint(data), " ".repeat(spacing))
}

pub fn cell_content_centered (data: String, span: usize, color: Colour) -> String {
	let str_len = data.len();
	let spacing = span - str_len;
	format!("{}{}{}", " ".repeat(spacing / 2), color.paint(data), " ".repeat((spacing / 2) + spacing % 2))
}

pub fn review_cell (review_opt: Option<ChangeReview>, width: usize) -> String {
    let (text, color) = match review_opt {
        Some(review) => match review.result {
            ReviewResult::Rejected => (format!("✗ {}", review.author).to_string(), Red),
            ReviewResult::Disliked => (format!("-1 {}", review.author).to_string(), Red),
            ReviewResult::Liked => (format!("+1 {}", review.author).to_string(), Green),
            ReviewResult::Approved => (format!("✓ {}", review.author).to_string(), Green),
            _ => (" ".to_string(), White),
        },
        None => (" ".to_string(), White)
    };

    cell_content_centered(text, width, color)
}
