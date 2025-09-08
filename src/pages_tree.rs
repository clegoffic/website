
use std::fs;

use tui_tree_widget::{Tree, TreeItem, TreeState};
use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType},
    Frame
};

#[derive(Default)]
pub struct PagesTree {
    state: TreeState<&'static str>,
    pages: Vec<TreeItem<'static, &'static str>>,
}

impl PagesTree {
    pub fn new() -> Self {
        let pages = vec![
            TreeItem::new_leaf("Home", "Home"),
            TreeItem::new_leaf("resume", "About"),
        ];

        Self {
            state: TreeState::default(),
            pages,
        }
    }

    pub fn get_selected(&self) -> Option<&'static str> {
        self.state.selected().get(0).cloned()
    }

    pub fn state_mut(&mut self) -> &mut TreeState<&'static str> {
        &mut self.state
    }

    pub fn draw(&mut self, frame: &mut Frame, area: Rect) {
        let widget = Tree::new(&self.pages)
            .expect("All pages are unique")
            .block(
                Block::bordered()
                    .title("Pages")
                    .title_alignment(Alignment::Center)
                    .border_type(BorderType::Rounded))
            .highlight_style(
                Style::default()
                    .bg(Color::LightGreen)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD));
        frame.render_stateful_widget(widget, area, &mut self.state);
    }
}