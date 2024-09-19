use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Intervals to cycle colour on (comma-separated seconds)
    #[arg(short, long)]
    pub intervals: Option<String>,

    /// Colours to represent each interval (comma-separated ANSI colours (0-7))
    #[arg(short, long)]
    pub colours: Option<String>,

    /// Count down to each interval boundary
    #[arg(short, long, default_value_t = false)]
    pub descending: bool,

    /// Pause on interval boundaries
    #[arg(short, long, default_value_t = false)]
    pub pause: bool,
}
