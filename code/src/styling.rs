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
                let styled_text = "Styling is easy".black().on_magenta();
                let bold_italic_text = "Look ma! Bold and italic".bold().italic();
                let mixed_line = vec![
                    "And ".fg(Color::Yellow),
                    "mixed".bg(Color::Indexed(1)),
                    " styling".fg(Color::Rgb(100, 200, 200)),
                ];
                let text: Vec<Line<'_>> = vec![
                    "".into(),
                    styled_text.into(),
                    bold_italic_text.into(),
                    mixed_line.into(),
                ];
                frame.render_widget(
                    Paragraph::new(text).centered().block(
                        Block::bordered()
                            .title("Styling text")
                            .title_alignment(Alignment::Center),
                    ),
                    frame.area(),
                );
            })
            .expect("failed to draw frame");

        if matches!(event::read().expect("failed to read event"), Event::Key(_)) {
            break;
        }
    }
    ratatui::restore();
}
