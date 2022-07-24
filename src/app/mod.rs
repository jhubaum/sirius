use crossterm::event::KeyCode;
use tui::{backend::Backend, Frame};

mod tasks;
use tasks::ProjectData;

mod display;
use display::ProjectDataEditView;

pub enum AppEvent {
    Exit,
}

pub struct App {
    view: ProjectDataEditView,
}

impl App {
    pub fn new() -> Self {
        let data = ProjectData::dummy_data();
        Self {
            view: ProjectDataEditView::new(data),
        }
    }

    pub fn on_tick(&mut self) -> Option<AppEvent> {
        None
    }

    pub fn on_key_pressed(&mut self, keycode: KeyCode) -> Option<AppEvent> {
        if self.view.handle_input(keycode) {
            return None;
        }

        match keycode {
            KeyCode::Char('q') => Some(AppEvent::Exit),
            KeyCode::Esc => Some(AppEvent::Exit),
            _ => None,
        }
    }

    pub fn render<B: Backend>(&mut self, f: &mut Frame<B>) {
        self.view.render(f);
    }
}
