use crossterm::event::KeyCode;
use std::collections::HashMap;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    text::{Span, Spans},
    widgets::Paragraph,
    Frame,
};

mod tasks;
use tasks::ProjectData;

mod display;
use display::ProjectDataEditView;

pub enum AppEvent {
    None,
    Exit,
    RequiresActionPopulation,
}

#[derive(Default)]
pub struct ActionPreviewSet {
    actions: HashMap<KeyCode, &'static str>,
}

impl ActionPreviewSet {
    fn reset(&mut self) {
        self.actions.clear();
    }

    pub fn insert(&mut self, key: KeyCode, desc: &'static str) {
        if self.actions.get(&key).is_none() {
            self.actions.insert(key, desc);
        }
    }

    fn to_text(&self) -> tui::text::Text {
        // TODO
        // Add styling
        // Take width parameter and fill spans with spaces (to reset lines)
        // Group descriptions (i.e. if multiple keys have same desc)
        // Prioritize actions (so that e.g. Exit always is the first printed command)
        // Add special case for Keycode::Char in format

        let spans: Spans = self
            .actions
            .iter()
            .map(|(key, desc)| Span::raw(format!("[{:?}] {}\t", key, desc)))
            .collect::<Vec<Span>>()
            .into();

        vec![spans].into()
    }
}

pub struct App {
    view: ProjectDataEditView,
    actions: ActionPreviewSet,
}

impl App {
    pub fn new() -> Self {
        let data = ProjectData::dummy_data();
        let mut app = Self {
            view: ProjectDataEditView::new(data),
            actions: ActionPreviewSet::default(),
        };
        app.populate_actions();
        app
    }

    pub fn populate_actions(&mut self) {
        self.actions.reset();

        self.view.populate_actions(&mut self.actions);

        self.actions.insert(KeyCode::Char('q'), "Exit");
        self.actions.insert(KeyCode::Esc, "Exit");
    }

    pub fn on_key_pressed(&mut self, keycode: KeyCode) -> AppEvent {
        if self.view.handle_input(keycode) {
            return AppEvent::RequiresActionPopulation;
        }

        match keycode {
            KeyCode::Char('q') => AppEvent::Exit,
            KeyCode::Esc => AppEvent::Exit,
            _ => AppEvent::None,
        }
    }

    pub fn render<B: Backend>(&mut self, f: &mut Frame<B>) {
        let action_menu = self.actions.to_text();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                vec![
                    Constraint::Length(f.size().height - (action_menu.height() as u16)),
                    Constraint::Length(action_menu.height() as u16),
                ]
                .as_ref(),
            )
            .split(f.size());

        self.view.render(f, chunks[0]);

        let action_menu = Paragraph::new(action_menu);
        f.render_widget(action_menu, chunks[1]);
    }
}
