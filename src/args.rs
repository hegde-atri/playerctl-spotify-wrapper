use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct SongArgs {
    /// Describe data to get.
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Debug, Subcommand)]
pub enum Action {
    /// Title of song
    #[clap(name = "title", about = "Current title")]
    Title(TitleArgs),
    /// Artist
    Artist,
    /// Next track,
    Next,
    /// Prev track
    Prev,
    /// Play-Pause
    Toggle,
    /// Toggle shuffle
    Shuffle,
    /// Loop
    Loop,
    /// Give artwork URL
    Art,
}

#[derive(Debug, Parser)]
pub struct TitleArgs {
    /// Give Title - Artist
    #[clap(long)]
    pub full: bool,
}
