use chrono::Datelike;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Widget},
};
use serde::Deserialize;
use std::io::{self, Write, stdout};

#[derive(Deserialize)]
struct Shortcut {
    key: String,
    description: String,
    category: String,
}

#[derive(Deserialize)]
struct ShortcutData {
    shortcuts: Vec<Shortcut>,
}

fn get_todays_shortcut(shortcuts: &[Shortcut]) -> &Shortcut {
    let today = chrono::Local::now();
    let day_of_year = today.ordinal0() as usize;
    let index = day_of_year % shortcuts.len();
    &shortcuts[index]
}

fn main() -> io::Result<()> {
    // Load shortcuts from the embedded JSON
    let shortcuts_json = r#"{
        "shortcuts": [
            {
                "key": "Ctrl+X,A",
                "description": "Expand alias under cursor",
                "category": "ZSH"
            },
            {
                "key": "Ctrl+X,Ctrl+E",
                "description": "Edit current command in $EDITOR",
                "category": "ZSH"
            },
            {
                "key": "Ctrl+X,Ctrl+X",
                "description": "Toggle between start of line and current position",
                "category": "ZSH"
            },
            {
                "key": "Ctrl+X,*",
                "description": "Expand word to all possible matches",
                "category": "ZSH"
            },
            {
                "key": "Ctrl+X,?",
                "description": "Get information about key bindings",
                "category": "ZSH"
            },
            {
                "key": "Ctrl+X,g",
                "description": "List possible glob expansions",
                "category": "ZSH"
            },
            {
                "key": "Ctrl+X,m",
                "description": "Repeat last word on previous line",
                "category": "ZSH"
            },
            {
                "key": "Ctrl+Alt+E",
                "description": "Expand alias under cursor (alternative method)",
                "category": "ZSH"
            },
            {
                "key": "Ctrl+X,Ctrl+V",
                "description": "Enter vi command mode",
                "category": "ZSH"
            },
            {
                "key": "Ctrl+X,Ctrl+U",
                "description": "Undo all changes to the line",
                "category": "ZSH"
            }
        ]
    }"#;

    let shortcut_data: ShortcutData = serde_json::from_str(shortcuts_json)?;
    let shortcut = get_todays_shortcut(&shortcut_data.shortcuts);

    // Create a virtual buffer for rendering
    let area = Rect::new(0, 0, 100, 8); // 100 chars wide, 8 lines tall
    let mut buffer = Buffer::empty(area);

    // Create the layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Length(1),  // Category
            Constraint::Length(3),  // Shortcut
            Constraint::Length(1),  // Description
        ])
        .split(area);

    // Render widgets to buffer
    let title = Paragraph::new("Today's Keyboard Shortcut")
        .style(Style::default().fg(Color::Cyan))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    title.render(chunks[0], &mut buffer);

    let category = Paragraph::new(format!("Category: {}", shortcut.category))
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center);
    category.render(chunks[1], &mut buffer);

    let shortcut_text = Paragraph::new(shortcut.key.clone())
        .style(Style::default().fg(Color::Green))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    shortcut_text.render(chunks[2], &mut buffer);

    let description = Paragraph::new(shortcut.description.clone())
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center);
    description.render(chunks[3], &mut buffer);

    // Print the buffer contents line by line
    let mut stdout = stdout();
    for y in 0..area.height {
        for x in 0..area.width {
            let cell = buffer.get(x, y);
            write!(
                stdout,
                "{}{}",
                crossterm::style::SetForegroundColor(cell.fg.into()),
                cell.symbol
            )?;
        }
        writeln!(stdout)?;
    }
    write!(stdout, "{}", crossterm::style::ResetColor)?;

    Ok(())
}