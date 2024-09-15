mod layout;

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
                let layouts = layout::get_layout_based_on_messages(60, frame);
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
