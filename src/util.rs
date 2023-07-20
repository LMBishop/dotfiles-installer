use crate::config::Step;
use colored::*;

pub const EXIT_IO_ERROR: i32 = 1;
pub const EXIT_INSTALL_FAILED: i32 = 2;

// todo: figure out a better way of doing this
pub fn str_step(step: &Step) -> String {
    match step {
        Step::Link(path) => format!("Link {} to {}", &path.from, &path.to),
        Step::Copy(path) => format!("Copy {} to {}", &path.from, &path.to),
        Step::Shell(command) => format!("Run {}", command),
    }
}

pub fn fmt_step(step: &Step, result: &Result<bool, Box<dyn std::error::Error>>) -> String {
    let action = str_step(step);

    match result {
        Ok(true) => { format!("{}{} {} {}", "[".bright_black(), "✔".green(), "]".bright_black(), action) }
        Ok(false) => { format!("{}{} {} {}", "[".bright_black(),  "▼".bright_black(), "]".bright_black(), action) }
        Err(_) => { format!("{}{} {} {}", "[".bright_black(), "✘".red(), "]".bright_black(), action) }
    }
}

pub fn expand_home(path: &str) -> String {
    let home = std::env::var("HOME").expect("no $HOME");
    if path.starts_with("~") {
        let rest_of_path = &path[1..];
        format!("{}{}", home, rest_of_path)
    } else {
        path.to_owned()
    }
}