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

#[cfg(test)]
mod tests {
    use ratatui::{buffer::Cell, layout::Position, prelude::*};

    #[test]
    fn assert_buffer() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 12, 4));
        buffer.set_string(0, 0, "Hello World!", Style::default());
        assert_eq!(
            buffer,
            Buffer::with_lines([
                "Hello World!",
                "            ",
                "            ",
                "            ",
            ])
        );
        assert_eq!(buffer.cell(Position::new(4, 0)), Some(&Cell::new("o")));
    }
}
