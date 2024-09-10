use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    prelude::*,
    widgets::*,
};

fn main() {
    let mut terminal = ratatui::init();
    let mut toggle = false;
    loop {
        terminal
            .draw(|frame: &mut Frame| {
                if toggle {
                    frame.render_widget(
                        BarChart::default()
                            .block(
                                Block::default()
                                    .borders(Borders::ALL)
                                    .title("Things That Give Me Power"),
                            )
                            .bar_width(10)
                            .bar_gap(10)
                            .group_gap(3)
                            .bar_style(
                                Style::default().fg(Color::Magenta).bg(Color::Black),
                            )
                            .value_style(
                                Style::default()
                                    .fg(Color::Magenta)
                                    .add_modifier(Modifier::BOLD),
                            )
                            .label_style(Style::default().fg(Color::White))
                            .data(&[("Money", 50), ("Status", 40), ("Rust", 100)])
                            .max(100),
                        frame.area(),
                    );
                } else {
                    frame.render_widget(
                        LineGauge::default()
                            .block(
                                Block::default()
                                    .borders(Borders::ALL)
                                    .title("Year Progress"),
                            )
                            .filled_style(
                                Style::default()
                                    .fg(Color::White)
                                    .bg(Color::Magenta)
                                    .add_modifier(Modifier::BOLD),
                            )
                            .line_set(symbols::line::THICK)
                            .ratio({
                                let current_day_of_year = 256;
                                let total_days_in_year = 365;
                                current_day_of_year as f64 / total_days_in_year as f64
                            }),
                        frame.area(),
                    );
                }
            })
            .expect("failed to draw frame");

        match event::read().expect("failed to read event") {
            Event::Key(key_event) => match key_event.code {
                KeyCode::Esc | KeyCode::Char('q') => break,
                KeyCode::Enter => toggle = !toggle,
                _ => {}
            },
            _ => {}
        }
    }
    ratatui::restore();
}
