use crate::state::{Break, Phase, AppState};
use ratatui::{
    layout::{Flex, Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph, Clear},
    style::{Color, Style, Modifier, Stylize},
    Frame,
};
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use anyhow::Result;
use chrono::Duration;

pub mod break_card;

pub fn render(f: &mut Frame, app_state: &AppState) { 
    match &app_state.phase {
        Phase::WorkSession => todo!(),
        Phase::StretchBreak | Phase::WaterBreak => {
            if let Ok(Some(b)) = app_state.get_break() {
                break_card::draw_break(f, &b);
            }
        }
    }
}





    
