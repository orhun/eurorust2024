use ratatui::prelude::*;
pub fn get_layout_based_on_messages(msg_count: usize, f: &Frame) -> std::rc::Rc<[Rect]> {
    let msg_percentage = if msg_count > 50 { 80 } else { 50 };
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(msg_percentage),
            Constraint::Percentage(100 - msg_percentage),
        ])
        .split(f.area())
}
