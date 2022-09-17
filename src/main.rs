use std::{env};
use std::convert::From;
use std::iter::Iterator;
use std::path::{PathBuf};
use std::result::Result::{Err, Ok};
use std::string::ToString;
use std::vec::Vec;


fn main() {
    let username = env::var("USER").unwrap();
    let cwd: PathBuf = env::current_dir().unwrap();
    let home_prefix: PathBuf = PathBuf::from(format!("/Users/{}", username));

    let mut display_path = cwd.display().to_string();
    if cwd.starts_with(&home_prefix) {
        display_path = cwd.strip_prefix(home_prefix).unwrap().display().to_string();
        display_path = format!("~/{}", display_path);
    }

    let mut output = format!("{}@{}", username, display_path);
    match env::var("VIRTUAL_ENV") {
        Ok(v) => {
            let venv_splat: Vec<&str> = v.split('/').collect();
            let venv_name = *(venv_splat.last().unwrap());
            output = format!("{} | {}", output, venv_name);
        }
        Err(_e) => {}
    }

    println!("{}", output)
}
