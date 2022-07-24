//! Types and functions to render tasks to the terminal.
//!
//! This module provides a public struct ProjectDataEditView that wraps
//! around a ProjectData instance and contains all information to render
//! and interact with the ProjectData via a TUI.

use crossterm::event::KeyCode;
use std::cmp;
use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

use super::ActionPreviewSet;
use super::tasks::{ProjectData, Task};

/// A trait to define all interactions with a task.
/// This is implemented as a trait to make it possible to have rendered
/// items that don't correspond to a Task from the ProjectData. This is
/// used, for example, to model the UI for creating a task.
trait TaskView {
    fn handle_input(&mut self, keycode: KeyCode) -> bool;
    fn create_widget(&self) -> ListItem;
    fn populate_actions(&self, actions: &mut ActionPreviewSet);
}

/// The central struct of this module.
/// It wraps around a ProjectData instance and contains all information to
/// render and interact with the ProjectData via a TUI.
pub struct ProjectDataEditView {
    tasks: Vec<Box<dyn TaskView>>,
    state: ListState,
}

impl ProjectDataEditView {
    pub fn new(data: ProjectData) -> Self {
        let mut view = Self {
            tasks: Vec::new(),
            state: ListState::default(),
        };
        for task in data.tasks.values() {
            view.tasks
                .push(Box::new(TaskViewImpl { task: task.clone() }))
        }
        view
    }

    pub fn populate_actions(&self, actions: &mut ActionPreviewSet) {
        if let Some(i) = self.state.selected() {
            self.tasks[i].populate_actions(actions);

            actions.insert(KeyCode::Esc, "Unselect task");
        }
    }

    pub fn handle_input(&mut self, keycode: KeyCode) -> bool {
        if let Some(i) = self.state.selected() {
            if self.tasks[i].handle_input(keycode) {
                return true;
            }
        }
        let mut handeled = true;

        match keycode {
            KeyCode::Esc => handeled = self.unselect(),
            KeyCode::Down => self.next(),
            KeyCode::Up => self.previous(),
            _ => handeled = false,
        };
        handeled
    }

    pub fn render<B: Backend>(&mut self, frame: &mut Frame<B>, rect: Rect) {
        let items: Vec<ListItem> = self.tasks.iter().map(|task| task.create_widget()).collect();

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
            );

        // We can now render the item list
        frame.render_stateful_widget(items, rect, &mut self.state);
    }

    fn next(&mut self) {
        self.state.select(Some(match self.state.selected() {
            Some(i) => cmp::min(self.tasks.len() - 1, i + 1),
            None => 0,
        }))
    }

    fn previous(&mut self) {
        self.state.select(Some(match self.state.selected() {
            Some(0) => 0,
            Some(i) => i - 1,
            None => self.tasks.len() - 1,
        }))
    }

    fn unselect(&mut self) -> bool {
        match self.state.selected() {
            Some(_) => {
                self.state.select(None);
                true
            }
            None => false,
        }
    }
}

/// A wrapper around a task containing all the state information
/// to interact with it in the terminal
struct TaskViewImpl {
    task: Task,
}

impl TaskView for TaskViewImpl {
    fn populate_actions(&self, actions: &mut ActionPreviewSet) {
        actions.insert(KeyCode::Enter, "Mark as done");
    }

    fn handle_input(&mut self, keycode: KeyCode) -> bool {
        let mut handeled = true;
        match keycode {
            KeyCode::Enter => self.task.done = true,
            _ => handeled = false,
        };
        handeled
    }

    fn create_widget(&self) -> ListItem {
        ListItem::new::<Spans>(
            vec![
                Span::styled("- ", Style::default().fg(Color::Blue)),
                Span::raw(format!("[{}] ", if self.task.done { "X" } else { " " })),
                Span::styled(
                    &self.task.description,
                    if self.task.done {
                        Style::default().fg(Color::Gray)
                    } else {
                        Style::default()
                    },
                ),
            ]
            .into(),
        )
    }
}
