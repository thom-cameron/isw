use std::{
    io,
    time::{Duration, Instant},
};

use crossterm::event::{self, poll, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{
        block::{Position, Title},
        Block, Paragraph, Widget,
    },
    DefaultTerminal, Frame,
};

#[derive(Debug)]
pub struct App {
    start_time: Instant,
    current_time: Duration,
    exit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            current_time: Duration::new(0, 0),
            exit: false,
        }
    }

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> (io::Result<()>, String) {
        while !self.exit {
            self.update_time();
            match terminal.draw(|frame| self.draw(frame)) {
                Err(err) => return (Err(err), self.get_formatted_time()),
                Ok(_) => {}
            };
            match self.handle_events() {
                Err(err) => return (Err(err), self.get_formatted_time()),
                Ok(_) => {}
            };
            // self.handle_events()?;
        }
        (Ok(()), self.get_formatted_time())
    }

    fn update_time(&mut self) { self.current_time = self.start_time.elapsed() }

    fn get_formatted_time(&self) -> String {
        let total_s = self.current_time.as_secs();

        let h = total_s / 3600;
        let m = (total_s % 3600) / 60;
        let s = total_s % 60;
        let ms = self.current_time.subsec_millis() / 100;

        format!("{:02}:{:02}:{:02}.{}", h, m, s, ms)
    }

    fn draw(&self, frame: &mut Frame) { frame.render_widget(self, frame.area()); }

    /// updates the application's state based on user input
    fn handle_events(&mut self) -> io::Result<()> {
        if !poll(Duration::from_secs(0))? {
            return Ok(());
        }
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }

    fn exit(&mut self) { self.exit = true; }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::new();

        let counter_text = Text::from(vec![Line::from(vec![
            // self.current_time.as_secs().to_string().green(),
            self.get_formatted_time().green(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
