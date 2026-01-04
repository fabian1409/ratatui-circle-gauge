use std::{f64::consts::PI, time::Duration};

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode},
    style::Style,
    text::Span,
    widgets::Widget,
};
use ratatui_circle_gauge::CircleGauge;

struct App {
    ratio: f64,
}

impl Default for App {
    fn default() -> Self {
        Self { ratio: 0.5 }
    }
}

fn main() -> eyre::Result<()> {
    let app = App::default();
    let terminal = ratatui::init();
    let result = app.run(terminal);
    ratatui::restore();
    result
}

impl App {
    fn run(mut self, mut terminal: DefaultTerminal) -> eyre::Result<()> {
        let mut change = 0.01;
        loop {
            terminal.draw(|frame| self.render(frame))?;

            // Non-blocking event polling with 16ms timeout (~60 FPS)
            if event::poll(Duration::from_millis(16))?
                && let Event::Key(key) = event::read()?
            {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    _ => {}
                }
            }

            self.ratio += change;

            if self.ratio >= 1.0 {
                self.ratio = 1.0;
                change = -change;
            } else if self.ratio <= 0.0 {
                self.ratio = 0.0;
                change = -change;
            }
        }
        Ok(())
    }

    fn render(&self, frame: &mut Frame) {
        let area = frame.area();
        let buf = frame.buffer_mut();
        CircleGauge::default()
            .ratio(self.ratio)
            .stroke(10.0)
            .fill_style(Style::new().red())
            .empty_style(Style::new().dark_gray())
            .start_angle(3.0 * PI / 2.0)
            .label(Span::raw(format!(
                "Loading... ({}%)",
                f64::round(self.ratio * 100.0)
            )))
            .render(area, buf);
    }
}
