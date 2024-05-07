use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Cell, Clear, List, ListItem, Paragraph, Row, Table, Wrap},
    Frame,
};

use crate::app::app::{App, CurrentScreen, CurrentlyEditingCredentialField};

mod footer;
mod helpers;
mod title;

const TITLE_CHUNK_INDEX: usize = 0;
const MAIN_CHUNK_INDEX: usize = 1;
const FOOTER_CHUNK_INDEX: usize = 2;

pub fn ui(f: &mut Frame, app: &App) {
    // Create the layout sections.
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(f.size());

    title::render_title(f, app, chunks[TITLE_CHUNK_INDEX]);

    match app.current_screen {
        CurrentScreen::MasterPasswordRequired => {
            render_password_required_screen(f, app, &chunks);
        }
        CurrentScreen::MainCredentialScreen => {
            render_main_credentials_screen(f, app, &chunks);
        }
        CurrentScreen::WebsiteCredentialScreen => {
            render_website_credentials_screen(f, app, &chunks);
        }
        CurrentScreen::SpecificCredentialScreen => {
            render_specific_credentials_screen(f, app, &chunks);
        }
        CurrentScreen::Exiting => {
            render_exit_screen(f, app, &chunks);
        }
    }

    footer::render_footer(f, app, chunks[FOOTER_CHUNK_INDEX]);
}

fn render_password_required_screen(f: &mut Frame, app: &App, chunks: &[Rect]) {
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

    let area = master_password_required_rect(50, f.size());
    f.render_widget(password_paragraph, area);
}

fn render_main_credentials_screen(f: &mut Frame, app: &App, chunks: &[Rect]) {
    let rows: Vec<Row> = app
        .websites
        .iter()
        .enumerate()
        .map(|(i, encoded_website_name)| {
            let website_name: String;
            let style: Style;
            if i == app.selected_website_index {
                // TODO display decoded website
                website_name = app.websites[i].clone();
                style = Style::default().bg(Color::LightYellow);
            } else {
                website_name = encoded_website_name.clone();
                style = Style::default();
            };

            Row::new(vec![Cell::from(Span::styled(
                format!("{: <1000}", website_name),
                style,
            ))])
        })
        .collect();

    let height = chunks[MAIN_CHUNK_INDEX].height - 2; // TODO: why is the height not equal to number of rows?
    let visible_rows = helpers::get_visible_rows(app.selected_website_index, rows, height);

    let widths = [Constraint::Length(5), Constraint::Length(5)];
    let table = Table::new(visible_rows, widths)
        .block(Block::default().borders(Borders::ALL))
        .widths(&[Constraint::Percentage(100)]);

    f.render_widget(table, chunks[MAIN_CHUNK_INDEX]);
}

fn render_website_credentials_screen(f: &mut Frame, app: &App, chunks: &[Rect]) {
    let rows: Vec<Row> = app
        .emails
        .iter()
        .enumerate()
        .map(|(i, email)| {
            let style: Style;
            if i == app.selected_email_index {
                style = Style::default().bg(Color::LightYellow);
            } else {
                style = Style::default();
            };

            Row::new(vec![Cell::from(Span::styled(
                format!("{: <1000}", email),
                style,
            ))])
        })
        .collect();

    let height = chunks[MAIN_CHUNK_INDEX].height - 2; // TODO: why is the height not equal to number of rows?
    let visible_rows = helpers::get_visible_rows(app.selected_email_index, rows, height);

    let widths = [Constraint::Length(5), Constraint::Length(5)];
    let table = Table::new(visible_rows, widths)
        .block(Block::default().borders(Borders::ALL))
        .widths(&[Constraint::Percentage(100)]);

    f.render_widget(table, chunks[MAIN_CHUNK_INDEX]);
}

fn render_specific_credentials_screen(f: &mut Frame, app: &App, chunks: &[Rect]) {
    if let Some(editing) = &app.currently_editing_credential_field {
        let popup_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default());

        f.render_widget(popup_block, chunks[MAIN_CHUNK_INDEX]);

        let popup_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Min(3),
                Constraint::Min(3),
                Constraint::Min(3),
                Constraint::Min(3),
                Constraint::Max(u16::MAX),
            ])
            .split(chunks[MAIN_CHUNK_INDEX]);

        let mut website_block = Block::default().title("Website").borders(Borders::ALL);
        let mut email_block = Block::default().title("Email").borders(Borders::ALL);
        let mut username_block = Block::default().title("Username").borders(Borders::ALL);
        let mut password_block = Block::default().title("Password").borders(Borders::ALL);
        let mut notes_block = Block::default().title("Notes").borders(Borders::ALL);

        let active_style = Style::default().bg(Color::LightYellow).fg(Color::Black);

        match editing {
            CurrentlyEditingCredentialField::Website => {
                website_block = website_block.style(active_style)
            }
            CurrentlyEditingCredentialField::Email => email_block = email_block.style(active_style),
            CurrentlyEditingCredentialField::Username => {
                username_block = username_block.style(active_style)
            }
            CurrentlyEditingCredentialField::Password => {
                password_block = password_block.style(active_style)
            }
            CurrentlyEditingCredentialField::Notes => notes_block = notes_block.style(active_style),
        };

        let website_text = Paragraph::new(app.website_input.clone()).block(website_block);
        f.render_widget(website_text, popup_chunks[0]);

        let email_text = Paragraph::new(app.email_input.clone()).block(email_block);
        f.render_widget(email_text, popup_chunks[1]);

        let username_text = Paragraph::new(app.username_input.clone()).block(username_block);
        f.render_widget(username_text, popup_chunks[2]);

        let password_text = Paragraph::new(app.password_input.clone()).block(password_block);
        f.render_widget(password_text, popup_chunks[3]);

        let notes_text = Paragraph::new(app.notes_input.clone()).block(notes_block);
        f.render_widget(notes_text, popup_chunks[4]);
    }
}

fn render_exit_screen(f: &mut Frame, app: &App, chunks: &[Rect]) {
    let popup_block = Block::default()
        .title("Unsaved Changes")
        .borders(Borders::NONE)
        .style(Style::default().bg(Color::Black));

    let exit_text = Text::styled(
        "Would you like to save your changes? (y/n)",
        Style::default().fg(Color::White),
    );
    // the `trim: false` will stop the text from being cut off when over the edge of the block
    let exit_paragraph = Paragraph::new(exit_text)
        .block(popup_block)
        .wrap(Wrap { trim: false });

    let area = centered_rect(60, 25, f.size());
    f.render_widget(exit_paragraph, area);
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

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
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
