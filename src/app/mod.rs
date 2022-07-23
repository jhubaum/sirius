use crossterm::event::KeyCode;

use tui::{
    backend::Backend,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

mod tasks;
use tasks::StatefulTaskView;

pub enum AppEvent {
    Exit,
}

pub struct App {
    tasks: StatefulTaskView,
}

impl App {
    pub fn new() -> Self {
        Self {
            tasks: StatefulTaskView::default(),
        }
    }

    pub fn on_tick(&mut self) -> Option<AppEvent> {
        None
    }

    pub fn on_key_pressed(&mut self, keycode: KeyCode) -> Option<AppEvent> {
        match keycode {
            KeyCode::Char('q') => return Some(AppEvent::Exit),
            KeyCode::Esc => self.tasks.unselect(),
            KeyCode::Down => self.tasks.next(),
            KeyCode::Up => self.tasks.previous(),
            KeyCode::Char('a') => self.tasks.insert_below_cursor(),
            KeyCode::Enter => self.tasks.mark_selected_as_done(),
            _ => {}
        };
        None
    }

    pub fn render<B: Backend>(&mut self, f: &mut Frame<B>) {
        let items: Vec<ListItem> = self
            .tasks
            .tasks
            .iter()
            .map(|task| {
                ListItem::new::<Spans>(
                    vec![
                        Span::styled("- ", Style::default().fg(Color::Blue)),
                        Span::raw(format!("[{}] ", if task.done { "X" } else { " " })),
                        Span::styled(&task.description, if task.done { Style::default().fg(Color::Gray) } else { Style::default() })
                    ]
                    .into(),
                )
            })
            .collect();

        // Create a List from all list items and highlight the currently selected one
        let items = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Sirus – Task Manager"),
            )
            .highlight_style(
                Style::default()
                    .bg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");

        // We can now render the item list
        f.render_stateful_widget(items, f.size(), &mut self.tasks.state);
    }
}
