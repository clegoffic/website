use std::{cell::RefCell, io, rc::Rc};

use ratatui::{
    Frame, Terminal,
    prelude::*,
    widgets::{Block, Borders, BorderType},
};

use ratzilla::{
    event::{KeyCode, KeyEvent},
    DomBackend, WebRenderer,
};

mod pages_tree;

fn main() -> io::Result<()> {
    let backend = DomBackend::new()?;
    let terminal = Terminal::new(backend)?;

    let state = Rc::new(App::new());

    let event_state = Rc::clone(&state);
    terminal.on_key_event(move |key_event| {
        event_state.handle_events(key_event);
    });

    let render_state = Rc::clone(&state);
    terminal.draw_web(move |frame| {
        render_state.render(frame);
    });

    Ok(())
}

#[derive(Default)]
struct App {
    pages_tree: RefCell<pages_tree::PagesTree>,
}

impl App {
    pub fn new() -> Self {
        Self {
            pages_tree: RefCell::new(pages_tree::PagesTree::new()),
        }
    }

    fn render(&self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(15),
                Constraint::Percentage(85),
            ])
            .split(frame.area());
        self.pages_tree.borrow_mut().draw(frame, layout[0]);

        let main_area = Block::default()
            .title("Main Area")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);
        let selected_page = self.pages_tree.borrow().get_selected();
        frame.render_widget(main_area, layout[1]);
        
    }

    fn handle_events(&self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Down => {
                self.pages_tree.borrow_mut().state_mut().key_down();
            }
            KeyCode::Up => {
                self.pages_tree.borrow_mut().state_mut().key_up();
            }
            _ => {}
        }
    }
}
