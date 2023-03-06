use std::{env};
use std::convert::From;
use std::iter::Iterator;
use std::option::Option::{None, Some};
use std::result::Result::{Err, Ok};
use std::string::{String, ToString};
use std::vec::Vec;
use git_info;
use chrono::{Local};
use nu_ansi_term::{Color};

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

    // command return code color
    let args: Vec<String> = env::args().collect();
    let ret_code: i16 = args.get(1).unwrap_or(&String::from("0")).parse().unwrap();
    let color_time = match ret_code {
        0 => {
            Color::LightGreen
        }
        1..=254 => {
            Color::Red
        }
        err => {
            println!("ret code not match {}", err);
            Color::Red
        }
    };

    // working directory
    let home_prefix = env::var("HOME").unwrap_or(String::from("no home dir"));
    if cwd.starts_with(&home_prefix) {
        cwd = format!("~{}", cwd.strip_prefix(&home_prefix).unwrap_or("-invalid cwd-"));
    }

    let mut prompt_items: Vec<String> = vec![];
    let current_time = Local::now().format("%H:%M:%S");
    prompt_items.push(color_time.bold().paint(current_time.to_string()).to_string());
    prompt_items.push(Color::Cyan.bold().paint(cwd).to_string());

    match env::var("VIRTUAL_ENV") {
        Ok(v) => {
            let venv_splat: Vec<&str> = v.split('/').collect();
            let venv_name = *(venv_splat.last().unwrap());
            let emoji_snake = '\u{1F40D}';
            //let venv_output = format!(" {} {} ", emoji_snake, venv_name);
            //prompt_items.push(String::from(""));
            prompt_items.push(Color::Yellow.paint(emoji_snake.to_string()).to_string());
            prompt_items.push(Color::Yellow.bold().paint(venv_name).to_string());
        }
        Err(_) => {}
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
            //let output_git = format!(" {} {} ", emoji_git, branch);
            //let output_git_color = Color::White.bold().paint(output_git).to_string();
            //prompt_items.push(String::from(""));
            prompt_items.push(emoji_git.to_string());
            prompt_items.push(Color::White.bold().paint(branch).to_string());
        }
        None => {}
    }

    println!("{}\n$", prompt_items.join(" "));
}
