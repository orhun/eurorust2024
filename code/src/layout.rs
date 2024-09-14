use std::rc::Rc;

use ratatui::{
    crossterm::event::{self, Event},
    prelude::*,
    widgets::*,
};

fn get_layout_based_on_messages(msg_count: usize, f: &Frame) -> Rc<[Rect]> {
    let msg_percentage = if msg_count > 50 { 80 } else { 50 };
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(msg_percentage),
            Constraint::Percentage(100 - msg_percentage),
        ])
        .split(f.area())
}

fn main() {
    let mut terminal = ratatui::init();
    loop {
        terminal
            .draw(|frame: &mut Frame| {
                let layouts = get_layout_based_on_messages(60, frame);
                for layout in layouts.iter() {
                    frame.render_widget(
                        Paragraph::new("Messages")
                            .block(Block::default().borders(Borders::ALL)),
                        *layout,
                    );
                }
            })
            .expect("failed to draw frame");

        if matches!(event::read().expect("failed to read event"), Event::Key(_)) {
            break;
        }
    }
    ratatui::restore();
}
