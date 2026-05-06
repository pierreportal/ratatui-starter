use crate::app::App;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Paragraph, Widget},
};

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Ratatui Starter ".bold());
        let instructions = Line::from(vec![" Quit ".into(), "<q> ".blue().bold()]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let sample_text = Line::from("Hello terminal");

        Paragraph::new(sample_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
