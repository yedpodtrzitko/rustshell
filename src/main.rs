use chrono::Local;
use git_info;
use nu_ansi_term::Color;
use std::convert::From;
use std::env;
use std::iter::Iterator;
use std::option::Option::Some;
use std::result::Result::{Err, Ok};
use std::string::{String, ToString};
use std::vec::Vec;

fn main() {
    let mut cwd = match env::current_dir() {
        Ok(dir) => dir.display().to_string(),
        Err(e) => {
            println!("{}", e);
            String::from("unknown dir")
        }
    };

    // command return code color
    let args: Vec<String> = env::args().collect();
    let ret_code: i16 = args.get(1).unwrap_or(&String::from("0")).parse().unwrap();
    let color_time = match ret_code {
        0 => Color::LightGreen,
        1..=254 => Color::Red,
        err => {
            println!("ret code not match {}", err);
            Color::Red
        }
    };

    // working directory
    let home_prefix = env::var("HOME").unwrap_or(String::from("no home dir"));
    if cwd.starts_with(&home_prefix) {
        cwd = format!(
            "~{}",
            cwd.strip_prefix(&home_prefix).unwrap_or("-invalid cwd-")
        );
    }

    let mut prompt_items: Vec<String> = vec![];
    let current_time = Local::now().format("%H:%M:%S").to_string();
    prompt_items.push(color_time.bold().paint(current_time).to_string());
    prompt_items.push(Color::Cyan.bold().paint(cwd).to_string());

    if let Ok(venv) = env::var("VIRTUAL_ENV") {
        let venv_splat: Vec<&str> = venv.split('/').collect();
        let venv_name = *(venv_splat.last().unwrap());
        let emoji_snake = '\u{1F40D}';
        prompt_items.push(Color::Yellow.paint(emoji_snake.to_string()).to_string());
        prompt_items.push(Color::Yellow.bold().paint(venv_name).to_string());
    }

    if let Some(git_branch) = git_info::get().current_branch {
        /*
        let emoji_dirty = if info.dirty.unwrap() {
            '\u{1F7E2}'
        } else {
            '\u{274C}'
        };*/
        let emoji_git = '\u{1F517}';
        prompt_items.push(emoji_git.to_string());
        prompt_items.push(Color::White.bold().paint(git_branch).to_string());
    }

    println!("{}\n$", prompt_items.join(" "));
}
