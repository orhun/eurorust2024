use rand::Rng;
use ratatui::prelude::*;

impl Widget for &mut crate::RandomColorWidget {
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
