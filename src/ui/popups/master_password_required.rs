use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::app::app::App;

pub fn render_master_password_required_popup(f: &mut Frame, app: &App) {
    let popup_block = Block::default()
        .title("Master Password")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::LightYellow));

    // the `trim: false` will stop the text from being cut off when over the edge of the block
    let password_str: String = std::iter::repeat('*')
        .take(app.master_password_input.len())
        .collect();
    let password_paragraph = Paragraph::new(password_str)
        .block(popup_block)
        .wrap(Wrap { trim: false });

    let area = master_password_required_rect(50, f.area());
    f.render_widget(password_paragraph, area);
}

fn master_password_required_rect(percent_x: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(u16::MAX),
            Constraint::Length(3),
            Constraint::Fill(u16::MAX),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
