//! ```cargo
//! [dependencies]
//! ratatui = "0.28.1"
//! ```

use ratatui::{
    crossterm::event::{self, Event},
    prelude::*,
    widgets::*,
};

fn main() {
    let mut terminal = ratatui::init();
    loop {
        terminal
            .draw(|frame: &mut Frame| {
                frame.render_widget(
                    Paragraph::new("Hello World!")
                        .centered()
                        .block(Block::bordered()),
                    frame.area(),
                )
            })
            .expect("failed to draw frame");

        if matches!(event::read().expect("failed to read event"), Event::Key(_)) {
            break;
        }
    }
    ratatui::restore();
}
