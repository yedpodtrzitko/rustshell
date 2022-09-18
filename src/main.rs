use std::{env};
use std::convert::From;
use std::iter::Iterator;
use std::option::Option::{None, Some};
use std::path::{PathBuf};
use std::result::Result::{Err, Ok};
use std::string::{String, ToString};
use std::vec::Vec;
use git_info;
use colored::*;
use chrono::{Local};

fn main() {
    let username = env::var("USER").unwrap();
    let cwd: PathBuf = env::current_dir().unwrap();
    let home_prefix: PathBuf = PathBuf::from(format!("/Users/{}", username));

    let mut output = cwd.display().to_string();
    if cwd.starts_with(&home_prefix) {
        output = cwd.strip_prefix(home_prefix).unwrap().display().to_string();
        output = format!("~/{}", output);
    }

    output = format!("{}@{} ", username, output).white().on_blue().to_string();

    let now = Local::now();
    let time = format!("{} ", now.format("%H:%M:%S")).to_string().black().on_white();
    output = format!("{}{}", time, output);
    //println!("{:02}:{:02}:{:02}", now.hour(), now.minute(), now.second());

    match env::var("VIRTUAL_ENV") {
        Ok(v) => {
            let venv_splat: Vec<&str> = v.split('/').collect();
            let venv_name = *(venv_splat.last().unwrap());
            let emoji_snake = '\u{1F40D}';
            let venv_output = format!(" {} {} ", emoji_snake, venv_name).on_yellow().black();
            output = format!("{}{}", output, venv_output)
        }
        Err(_e) => {}
    }

    let info = git_info::get();
    match info.current_branch {
        Some(branch) => {
            let emoji_git = '\u{1F517}';
            /*
            let emoji_dirty = if info.dirty.unwrap() {
                '\u{1F7E2}'
            } else {
                '\u{274C}'
            };*/
            let output_git = format!(" {} {} ", emoji_git, branch).black().on_white().to_string();
            output = format!("{}{}", output, output_git)
        }
        None => {}
    }

    // exit code
    println!("ret code: {}", env::var("?").unwrap_or(String::from("-")));

    println!("{}", output)
}
