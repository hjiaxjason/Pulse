use crate::state::{Break, Phase, AppState};
use ratatui::{
    layout::{Flex, Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph, Clear},
    style::{Color, Style, Modifier, Stylize},
    text::{Text, Line},
    Frame,
};
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use tui_big_text::{BigText, PixelSize};
use chrono::Duration;

/// Scale glyph-grid art up by an integer `factor`, keeping proportions exact:
/// each cell is repeated `factor` times across and each row `factor` times down.
fn scale_art(rows: &[&str], factor: usize) -> Vec<Line<'static>> {
    rows.iter()
        .flat_map(|row| {
            let wide: String = row
                .chars()
                .flat_map(|c| std::iter::repeat(c).take(factor))
                .collect();
            std::iter::repeat(wide).take(factor)
        })
        .map(Line::from)
        .collect()
}

pub fn draw_break(f: &mut Frame, break_type: &Break) {
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(vec![
            Constraint::Percentage(55),
            Constraint::Percentage(45)])
        .split(f.area());

    if *break_type == Break::Water {
        draw_water_break_header(f, outer_layout[0]);
    } else if *break_type == Break::Stretch {
        draw_stretch_break_header(f, outer_layout[0]);
    }

    f.render_widget(
        Paragraph::new("\nTime for a BREAK")
        .block(
            Block::new().title(" NUDGE ").bold().fg(Color::White).borders(Borders::ALL)),
        outer_layout[1],
    );
}

fn draw_water_break_header(f: &mut Frame, area: Rect) {
    let block = Block::new()
        .borders(Borders::ALL)
        .border_style(Style::new().white().bold());

    let inner = block.inner(area);
    f.render_widget(block, area);

    let [text_area, right_area] = Layout::horizontal([
        Constraint::Percentage(65),
        Constraint::Percentage(35),
    ])
    .areas(inner);

    let lines = vec![
        " Time for you to".white().into(),
        "   DRINK WATER".blue().into(),
    ];
    let text_height = lines.len() as u16 * 8; // 8 terminal rows per line at PixelSize::Full
    let text = BigText::builder()
        .pixel_size(PixelSize::Full)
        .style(Style::new().blue())
        .lines(lines)
        .build();

    // Vertically center the big text within its column.
    let [text_center] = Layout::vertical([Constraint::Length(text_height)])
        .flex(Flex::Center)
        .areas(text_area);
    f.render_widget(text, text_center);
    let base_art = [
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
        "⠀⠀⠀⠀⠀⠀⠀⠿⠿⠿⠿⠇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
        "⠀⠀⠀⠀⠀⠀⢀⣴⣶⣶⣶⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
        "⠀⠀⠀⠀⠀⢠⣾⣿⣿⣿⣿⣿⣧⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
        "⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
        "⠀⠀⠀⠀⠀⠛⠛⠛⠛⠛⠛⠛⠛⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
        "⠀⠀⠀⠀⠀⠚⠛⠛⠛⠛⠛⠛⠛⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
        "⠀⠀⠀⠀⠀⠛⠛⠛⠛⠛⠛⠛⠛⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
        "⠀⠀⠀⠀⠀⠲⠶⠶⠶⠶⠶⠶⠶⠆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
        "⠀⠀⠀⠀⠀⣶⣶⣶⣶⣶⣶⣶⣶⡆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
        "⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⠀⠀⢸⡿⠿⣿⣿⣿⡷⠀⠀⠀⠀⠀",
        "⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⠀⠀⠀⣧⠀⣿⣿⣿⡇⠀⠀⠀⠀⠀",
        "⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⠀⠀⠀⣿⠀⣿⣿⣿⡇⠀⠀⠀⠀⠀",
        "⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⠀⠀⠀⣿⠀⣿⣿⣿⠀⠀⠀⠀⠀⠀",
        "⠀⠀⠀⠀⠀⠈⠉⠉⠉⠉⠉⠉⠉⠀⠀⠀⠀⠀⠀⠉⠀⠉⠉⠉⠀⠀⠀⠀⠀⠀",
    ];
    let drop = Text::from(scale_art(&base_art, 2));
    // Carve an exact-size rect centered in `right_area` on both axes, then draw
    // the art into it untouched so its own internal spacing is preserved.
    let art_width = drop.width() as u16;
    let art_height = drop.height() as u16;
    let [centered_row] = Layout::vertical([Constraint::Length(art_height)])
        .flex(Flex::Center)
        .areas(right_area);
    let [drop_area] = Layout::horizontal([Constraint::Length(art_width)])
        .flex(Flex::Center)
        .areas(centered_row);

    f.render_widget(Paragraph::new(drop).blue(), drop_area);
}
    
fn draw_stretch_break_header(f: &mut Frame, area: Rect) {
    let block = Block::new()
        .borders(Borders::ALL)
        .border_style(Style::new().white().bold());

    let inner = block.inner(area);
    f.render_widget(block, area);

    let lines = vec![
        " Time for a BREAK".white().into(),
        "   Get up and stretch!!!".yellow().into(),
    ];
    let text_height = lines.len() as u16 * 8; // 8 terminal rows per line at PixelSize::Full
    let text = BigText::builder()
        .pixel_size(PixelSize::Full)
        .style(Style::new().blue())
        .lines(lines)
        .build();

    // Vertically center the big text within the card.
    let [text_center] = Layout::vertical([Constraint::Length(text_height)])
        .flex(Flex::Center)
        .areas(inner);
    f.render_widget(text, text_center);
}


