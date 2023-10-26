use clap::{Command, Arg};
use colored::Colorize;

fn main() {
    let matches = Command::new("bwd")
        .version("0.1.0")
        .about("A modern clone of linux pwd command with some additional features.")
        .author("BajrangCoder")
        .arg(
            Arg::new("show_path")
                .short('p')
                .long("show_path")
                .num_args(0..1)
                .help("command to show path of current directory path")
        )
        .get_matches();
    if matches.get_flag("show_path") {
        if let Some(pwd) = std::env::var("PWD").ok() {
            println!("{}", formated_output(&pwd));
        } else {
            if let Ok(current_dir) = std::env::current_dir() {
                println!("{}", formated_output(&current_dir.to_string_lossy()));
            }
        }
    } else {
        if let Some(pwd) = std::env::var("PWD").ok() {
            println!("{}", formated_output(&pwd));
        } else {
            if let Ok(current_dir) = std::env::current_dir() {
                println!("{}", formated_output(&current_dir.to_string_lossy()));
            }
        }
    }
}

fn formated_output(path: &str) -> String {
    path.green().to_string()
}