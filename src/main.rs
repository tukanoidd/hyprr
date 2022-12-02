mod cli;
mod gui;

use clap::Parser;
use color_eyre::eyre;

use crate::cli::Commands;

#[derive(clap::Parser)]
#[command(
    author,
    version,
    about,
    long_about = Some("hyprctl reimplementation in Rust with additional functionality")
)]
struct Args {
    #[arg(short, long, exclusive(true))]
    gui: bool,

    #[arg(short, long)]
    json: bool,

    #[arg(long)]
    batch: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    std::env::set_var("RUST_LOG", "info");
    pretty_env_logger::init_timed();

    let Args {
        gui,
        json,
        batch,
        command,
    } = Args::parse();

    match gui {
        true => gui::execute(),
        false => cli::execute(batch, command, json),
    }
}
