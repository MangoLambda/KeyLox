use crate::app::app::{App, CurrentScreen};

use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_footer(f: &mut Frame, app: &App, area: Rect) {
    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Init => Span::styled(
                "", Style::default()
            ),
            CurrentScreen::NewPasswordRequiredScreen => Span::styled(
                "Enter New Password", Style::default().fg(Color::Red)
            ),
            CurrentScreen::MasterPasswordRequiredScreen => Span::styled("Enter Master Password", Style::default().fg(Color::Red)
            ),
            CurrentScreen::MainCredentialScreen => Span::styled(
                "(Esc) to quit / (N) to make a new credential / (Return) to view",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::WebsiteCredentialScreen => Span::styled(
                "(Esc) to quit / (N) to make a new credential / (Return) to view / (Backspace) to delete",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::SpecificCredentialScreen => Span::styled(
                "(Esc) to quit / (Return | ^S) to save",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Exiting => Span::styled(
                "(Esc | Return | Q) to quit without saving / (W | S) to save and quit",
                Style::default().fg(Color::Red),
            ),
        }
    };

    let key_notes_footer =
        Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL));

    f.render_widget(key_notes_footer, area);
}
