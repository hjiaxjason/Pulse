use crate::state::AppState;
use ratatui::{
    layout::{Flex, Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph, Clear},
    style::{Color, Style, Modifier, Stylize},
    Frame,
};
use tui_big_text::{BigText, PixelSize};

pub fn draw_water_break(f: &mut Frame, state: &AppState) {
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(vec![
            Constraint::Percentage(35),
            Constraint::Percentage(65)])
        .split(f.area());

    draw_water_break_header(f, outer_layout[0]);

    f.render_widget(
        Paragraph::new("\nTime for a WATER BREAK")
        .block(
            Block::new().title(" NUDGE ").bold().fg(Color::Blue).borders(Borders::ALL)),
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
        Constraint::Percentage(70),
        Constraint::Percentage(30),
    ])
    .areas(inner);

    let text = BigText::builder()
        .pixel_size(PixelSize::Full)
        .style(Style::new().blue())
        .lines(vec![
            "Time for your".white().into(),
            "WATER BREAK".blue().into(),
        ])
        .build();

    f.render_widget(text, text_area);
    const DROP: &str = "\
    💧  💧     💧 💧  💧
      💧   💧      💧
    💧    💧 💧       💧
       💧        💧
    💧     💧        💧
          💧    .
      💧  .    💧
      .           .
          💧 .
        .        💧    .
        💧   . 
          💧 .
        . 
        .
    ";
    let height = DROP.lines().count() as u16;
    let [drop_area] = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .areas(right_area);

    f.render_widget(Paragraph::new(DROP).cyan().centered(), drop_area);
}
    

    
