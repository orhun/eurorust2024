use ratatui::{
    crossterm::event::{self, Event},
    prelude::*,
};

pub fn render(mut terminal: Terminal<CrosstermBackend<std::io::Stdout>>) {
    let mut random_color_widget = crate::RandomColorWidget {
        rng: rand::thread_rng(),
    };
    loop {
        terminal
            .draw(|frame: &mut Frame| {
                frame.render_widget(&mut random_color_widget, frame.area())
            })
            .expect("failed to draw frame");

        if event::poll(std::time::Duration::from_millis(50))
            .expect("failed to poll event")
        {
            if matches!(event::read().expect("failed to read event"), Event::Key(_)) {
                break;
            }
        }
    }
}
