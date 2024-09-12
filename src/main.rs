use std::io;

use clap;

use ratatui;
use sw::App;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let (app_result, final_time) = App::new().run(&mut terminal);
    ratatui::restore();
    println!("{}", final_time);
    app_result
}
