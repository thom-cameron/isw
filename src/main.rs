use std::{io, process::exit};

mod intervals;
use intervals::IntervalList;
mod cli;
mod stopwatch;
mod tui;
use crate::cli::Args;
use crate::stopwatch::Stopwatch;
use tui::App;

use clap::Parser;

fn main() -> io::Result<()> {
    // try to prevent the user's machine from sleeping
    let _ = keepawake::Builder::default()
        .display(true)
        .idle(true)
        .sleep(true)
        .create();

    // parse cli arguments and determine interval behaviour
    let args = Args::parse();
    let intervals: Option<IntervalList> = match args.intervals {
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

    // initialize the tui and stopwatch and start the app
    let mut terminal = ratatui::init();
    let stopwatch = Stopwatch::new(
        intervals,
        args.descending,
        args.pause,
        args.shell,
        args.show_interval,
        args.show_cycle,
    );
    let (app_result, final_time) = App::new(stopwatch).run(&mut terminal);
    ratatui::restore();

    // print the final time to stdout and exit
    println!("{final_time}");
    app_result
}
