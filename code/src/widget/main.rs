mod render;
mod widget;

pub struct RandomColorWidget {
    rng: rand::rngs::ThreadRng,
}

fn main() {
    let terminal = ratatui::init();
    render::render(terminal);
    ratatui::restore();
}
