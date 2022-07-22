use std::cmp;
use tui::widgets::ListState;

pub struct Task {
    // change this to timestamp asap
    // also use a task state instead of a boolean
    pub done: bool,
    pub description: String,
}

#[derive(Default)]
pub struct StatefulTaskView {
    pub state: ListState,
    pub tasks: Vec<Task>,
}

impl StatefulTaskView {
    pub fn insert_below_cursor(&mut self) {
        let new = Task {
            done: false,
            description: format!("Test {}", self.tasks.len()),
        };
        match self.state.selected() {
            None => self.tasks.push(new),
            Some(i) => self.tasks.insert(i, new),
        }
    }

    pub fn mark_selected_as_done(&mut self) {
        if let Some(i) = self.state.selected() {
            self.tasks[i].done = true;
        }
    }

    pub fn next(&mut self) {
        self.state.select(Some(match self.state.selected() {
            Some(i) => cmp::min(self.tasks.len() - 1, i + 1),
            None => 0,
        }))
    }

    pub fn previous(&mut self) {
        self.state.select(Some(match self.state.selected() {
            Some(0) => 0,
            Some(i) => i - 1,
            None => self.tasks.len() - 1,
        }))
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}
