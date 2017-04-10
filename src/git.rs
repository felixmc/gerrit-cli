use std::io::Error;
use exec::*;

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

    pub fn change_id (&self) -> String {
        "".to_owned()
    }
}
