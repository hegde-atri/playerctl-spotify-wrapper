use std::{
    io::Error,
    process::{Command, Output},
};

use args::TitleArgs;
use clap::Parser;

use crate::args::{Action, SongArgs};

mod args;

fn main() {
    let args = SongArgs::parse();
    match args.action {
        Action::Title(title_args) => get_title(title_args),
        Action::Artist => get_artist(),
        Action::Toggle => play_pause(),
        Action::Shuffle => toggle_shuffle(),
        Action::Loop => cycle_loop(),
        Action::Art => get_art(),
    }
}

fn get_title(title_args: TitleArgs) {
    if title_args.full {
        println!(
            "{} - {}",
            get_string_output(exec("playerctl --player spotify metadata xesam:title").unwrap()),
            get_string_output(exec("playerctl --player spotify metadata xesam:artist").unwrap())
        )
    } else {
        println!(
            "{}",
            get_string_output(exec("playerctl --player spotify metadata xesam:title").unwrap()),
        )
    }
}

fn get_artist() {
    println!(
        "{}",
        get_string_output(exec("playerctl --player spotify metadata xesam:artist").unwrap())
    )
}

fn play_pause() {
    exec("playerctl --player spotify play-pause").unwrap();
}

fn cycle_loop() {
    match get_string_output(exec("playerctl --player spotify loop").unwrap()).as_str() {
        "None" => {
            exec("playerctl --player spotify loop Playlist").unwrap();
            println!("Playlist")
        }
        "Playlist" => {
            exec("playerctl --player spotify loop Track").unwrap();
            println!("Track")
        }
        "Track" => {
            exec("playerctl --player spotify loop None").unwrap();
            println!("None")
        }
        _ => println!(""),
    };
}

fn toggle_shuffle() {
    exec("playerctl --player spotify shuffle Toggle").unwrap();
}

fn get_art() {
    exec("playerctl --player spotify metadata mpris:artUrl").unwrap();
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
