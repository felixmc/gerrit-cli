// fn git_status () -> Result<String, Error> {
//     let command = Command::new("git")
//                           .arg("log")
//                           .output();
//
//     command.map(|output| output.stdout.iter().fold("".to_owned(), |sum, val| format!("{}{}", sum, *val as char)))
// }

// match git_status() {
//     Ok(status) => println!("Git Status: {}", status),
//     Err(e) => println!("Error: {:?}", e)
// }
