use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Cell, Clear, List, ListItem, Paragraph, Row, Table, Widget, Wrap},
    Frame,
};

use crate::app::app::{App, CurrentScreen, CurrentlyEditingCredentialField};

const TITLE_CHUNK_INDEX: usize = 0;
const MAIN_CHUNK_INDEX: usize = 1;
const FOOTER_CHUNK_INDEX: usize = 2;

pub fn render_footer(f: &mut Frame, app: &App, chunks: &[Rect]) {
    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::MasterPasswordRequired => {
                Span::styled("Enter Master Password", Style::default().fg(Color::Red))
            }
            CurrentScreen::MainCredentialScreen => Span::styled(
                "(Esc) to quit / (N) to make a new credential / (Return) to view",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::WebsiteCredentialScreen => Span::styled(
                "(Esc) to quit / (N) to make a new credential / (Return) to view / (Backspace) to delete",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::SpecificCredentialScreen => Span::styled(
                "(Esc) to quit / (Return | Ctrl+S) to save",
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

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)])
        .split(chunks[FOOTER_CHUNK_INDEX]);

    // TODO: method should return widget instead?
    f.render_widget(key_notes_footer, footer_chunks[0]);
}
