use chrono::Local;
use nu_ansi_term::Color;
use std::env;

fn get_return_code_color() -> Color {
    let ret_code: i16 = env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(0);

    match ret_code {
        0 => Color::LightGreen,
        1..=254 => Color::Red,
        _ => Color::Red,
    }
}

fn get_working_directory() -> String {
    let cwd = match env::current_dir() {
        Ok(dir) => dir.display().to_string(),
        Err(_) => return "unknown dir".to_string(),
    };

    // Optimize home directory replacement - handle potential edge cases
    if let Ok(home_prefix) = env::var("HOME") {
        if !home_prefix.is_empty() && cwd.starts_with(&home_prefix) {
            if let Some(suffix) = cwd.strip_prefix(&home_prefix) {
                return format!("~{}", suffix);
            }
        }
    }
    cwd
}

fn add_venv_info(prompt_items: &mut Vec<String>) {
    if let Ok(venv) = env::var("VIRTUAL_ENV") {
        if !venv.is_empty() {
            // Use rfind for better performance than split().last()
            if let Some(slash_pos) = venv.rfind('/') {
                let venv_name = &venv[slash_pos + 1..];
                if !venv_name.is_empty() {
                    prompt_items.push(Color::Yellow.paint("🐍").to_string());
                    prompt_items.push(Color::Yellow.bold().paint(venv_name).to_string());
                }
            } else if !venv.is_empty() {
                // Handle case where there's no slash in the path
                prompt_items.push(Color::Yellow.paint("🐍").to_string());
                prompt_items.push(Color::Yellow.bold().paint(&venv).to_string());
            }
        }
    }
}

fn add_git_info(prompt_items: &mut Vec<String>) {
    if let Some(git_branch) = git_info::get().current_branch {
        prompt_items.push("🔗".to_string());
        prompt_items.push(Color::White.bold().paint(git_branch).to_string());
    }
}

fn main() {
    let color_time = get_return_code_color();
    let cwd = get_working_directory();

    // Pre-allocate with exact capacity we might need
    let mut prompt_items = Vec::with_capacity(8);

    // Add time with optimized formatting
    let current_time = Local::now().format("%H:%M:%S");
    prompt_items.push(
        color_time
            .bold()
            .paint(current_time.to_string())
            .to_string(),
    );
    prompt_items.push(Color::Cyan.bold().paint(cwd).to_string());

    add_venv_info(&mut prompt_items);
    add_git_info(&mut prompt_items);

    println!("{}\n$", prompt_items.join(" "));
}
