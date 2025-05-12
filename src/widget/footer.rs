use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    text::Line,
    widgets::{Block, Paragraph, Widget},
};

pub struct Footer {
    // pub instructions: Vec<String, char,
}

impl Widget for Footer {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let instructions = Line::from(vec![" Quit ".into(), "<q> ".blue().bold()]);
        let block = Block::new().title_bottom(instructions.centered());

        Paragraph::new("").centered().block(block).render(area, buf);
    }
}
