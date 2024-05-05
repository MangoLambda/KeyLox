use ratatui::widgets::Row;

pub fn get_visible_rows(selected_index: usize, rows: Vec<Row>, height: u16) -> Vec<Row> {
    let first_item_in_row = selected_index - (selected_index % height as usize);
    let last_item_in_row = std::cmp::min(first_item_in_row + height as usize, rows.len() - 1);
    rows[first_item_in_row..=last_item_in_row].to_vec()
}
