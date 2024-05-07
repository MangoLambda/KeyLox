use ratatui::{
    layout::Rect,
    style::Style,
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_title(f: &mut Frame, area: Rect) {
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title =
        Paragraph::new(Text::styled("Password Manager", Style::default())).block(title_block);

    f.render_widget(title, area);
}
