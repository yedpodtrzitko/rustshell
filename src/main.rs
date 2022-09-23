use std::{env};
use std::convert::From;
use std::iter::Iterator;
use std::option::Option::{None, Some};
use std::path::{PathBuf};
use std::result::Result::{Err, Ok};
use std::string::{String, ToString};
use std::vec::Vec;
use git_info;
use chrono::{Local};

fn main() {
    let mut cwd = match env::current_dir() {
        Ok(dir) => {
            dir.display().to_string()
        }
        Err(e) => {
            println!("{}", e);
            String::from("unknown dir")
        }
    };

    let username = env::var("USER").unwrap_or(String::from("no username"));
    let home_path: PathBuf = ["/Users/", username.as_str()].iter().collect();
    let home_prefix = home_path.to_str().unwrap();
    if cwd.starts_with(home_prefix) {
        cwd = format!("~{}", cwd.strip_prefix(home_prefix).unwrap());
    }

    let current_time =  Local::now().format("%H:%M:%S");//.to_string();
    let mut output = format!("{} {}@{} ", current_time, username, cwd);

    match env::var("VIRTUAL_ENV") {
        Ok(v) => {
            let venv_splat: Vec<&str> = v.split('/').collect();
            let venv_name = *(venv_splat.last().unwrap());
            let emoji_snake = '\u{1F40D}';
            let venv_output = format!(" {} {} ", emoji_snake, venv_name);//.on_yellow().black();
            output = format!("{}{}", output, venv_output)
        }
        Err(e) => {
            println!("virtualenv error {}", e);
        }
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
            let output_git = format!(" {} {} ", emoji_git, branch);//.black().on_white().to_string();
            output = format!("{}{}", output, output_git)
        }
        None => {}
    }

    let args: Vec<String> = env::args().collect();
    let ret_code: i8 = args.get(1).unwrap_or(&String::from("0")).parse().unwrap();
    let ret_emoji = match ret_code {
        0 => {
            '\u{1F7E2}'
        }
        1..=127 => {
            '\u{274C}'
        }
        _ => {
            println!("no ret code");
            '\u{274C}'
        }
    };

    output = format!("{} {}\n $", output, ret_emoji);
    println!("{}", output)
}
