#![feature(box_syntax)]

mod cli;
mod gui;

use color_eyre::eyre;
use itertools::Itertools;

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    std::env::set_var("RUST_LOG", "info");
    pretty_env_logger::init_timed();

    let args = std::env::args().collect_vec();

    if args.len() <= 1 {
        return Err(eyre::eyre!("{}", cli::USAGE));
    }

    let args = &args[1..];
    dbg!(&args);

    if ["-g", "--gui"].contains(&args[0].as_str()) {
        gui::execute()
    } else {
        cli::execute(args)
    }
}
