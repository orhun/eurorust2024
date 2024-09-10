use std::time;

use rand::Rng;
use ratatui::{
    crossterm::event::{self, Event},
    prelude::*,
};

pub struct RandomColorWidget {
    rng: rand::rngs::ThreadRng,
}

impl Widget for &mut RandomColorWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        for x in area.left()..area.right() {
            for y in area.top()..area.bottom() {
                let color = Color::from_hsl(
                    self.rng.gen_range(0..=10) as f64,
                    self.rng.gen_range(0..=10) as f64,
                    self.rng.gen_range(0..=40) as f64,
                );
                if let Some(cell) = buf.cell_mut(Position::new(x, y)) {
                    cell.reset();
                    cell.set_bg(color);
                }
            }
        }
    }
}

fn main() {
    let mut terminal = ratatui::init();
    loop {
        let mut random_color_widget = RandomColorWidget {
            rng: rand::thread_rng(),
        };
        terminal
            .draw(|frame: &mut Frame| {
                frame.render_widget(&mut random_color_widget, frame.area())
            })
            .expect("failed to draw frame");

        if event::poll(time::Duration::from_millis(50)).expect("failed to poll event") {
            if matches!(event::read().expect("failed to read event"), Event::Key(_)) {
                break;
            }
        }
    }
    ratatui::restore();
}
