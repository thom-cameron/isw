use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, author, about, long_about = None)]
pub struct Args {
    /// Intervals to cycle colour on (comma-separated seconds)
    #[arg(short, long, verbatim_doc_comment)]
    pub intervals: Option<String>,

    /// Colours to represent each interval
    /// (comma-separated ANSI colours (0-7))
    #[arg(short, long, verbatim_doc_comment)]
    pub colours: Option<String>,

    /// Count down to each interval boundary
    #[arg(short, long, verbatim_doc_comment, default_value_t = false)]
    pub descending: bool,

    /// Pause on interval boundaries (p or space to unpause)
    #[arg(short, long, verbatim_doc_comment, default_value_t = false)]
    pub pause: bool,

    /// Execute a shell command at the end of intervals
    /// ("%i" for interval and "%c" for cycle number)
    #[arg(short, long, verbatim_doc_comment)]
    pub shell: Option<String>,

    /// Show the number of intervals elapsed
    #[arg(long, verbatim_doc_comment, default_value_t = false)]
    pub show_interval: bool,

    /// Show the current number of interval cycles elapsed
    #[arg(long, verbatim_doc_comment, default_value_t = false)]
    pub show_cycle: bool,
}
