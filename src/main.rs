use std::{
    io::Error,
    process::{Command, Output},
};

fn main() {
    println!(
        "{} - {}",
        get_string_output(exec("playerctl --player spotify metadata title").unwrap()),
        get_string_output(exec("playerctl --player spotify metadata artist").unwrap())
    )
}

fn exec(cmd: &str) -> Result<Output, Error> {
    Command::new("sh")
        .current_dir("./")
        .arg("-c")
        .arg(cmd)
        .output()
}

fn get_string_output(output: Output) -> String {
    String::from_utf8(output.stdout).unwrap().trim().to_string()
}
