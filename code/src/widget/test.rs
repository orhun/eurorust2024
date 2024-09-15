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
