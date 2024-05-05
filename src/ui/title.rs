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

pub fn render_title(f: &mut Frame, app: &App, chunks: &[Rect]) {
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title =
        Paragraph::new(Text::styled("Password Manager", Style::default())).block(title_block);

    f.render_widget(title, chunks[TITLE_CHUNK_INDEX]);
}
