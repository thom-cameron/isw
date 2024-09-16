use std::{io, process::exit};

use clap::Parser;
mod intervals;
use intervals::IntervalList;

use ratatui;
mod tui;
use tui::App;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Intervals to cycle colour on (comma-separated seconds)
    #[arg(short, long)]
    durations: Option<String>,

    /// Colours to represent each interval (comma-separated ANSI colours (0-7))
    #[arg(short, long)]
    colours: Option<String>,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let intervals: Option<IntervalList> = match args.durations {
        Some(durations) => match IntervalList::new(durations, args.colours) {
            Ok(intervals) => Some(intervals),
            Err(e) => {
                eprintln!("{e}");
                exit(1);
            }
        },
        None => {
            if args.colours.is_some() {
                eprintln!(
                    "Colours were provided but no durations. The stopwatch will not use intervals."
                );
            };
            None
        }
    };
    println!("{:#?}", intervals);

    let mut terminal = ratatui::init();
    let (app_result, final_time) = App::new().run(&mut terminal);
    ratatui::restore();
    println!("{final_time}");

    app_result
}
