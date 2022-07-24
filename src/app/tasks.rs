use std::collections::HashMap;

type TaskID = usize;

#[derive(Clone)]
pub struct Task {
    id: TaskID,
    // change this to timestamp asap
    // also use a task state instead of a boolean
    pub done: bool,
    pub description: String,
}

#[derive(Default)]
pub struct ProjectData {
    pub tasks: HashMap<TaskID, Task>,
}

impl ProjectData {
    pub fn dummy_data() -> Self {
        let mut data = Self::default();
        data.insert_task(String::from("This is a test task"));
        data.insert_task(String::from("This is another test task"));
        data
    }

    pub fn insert_task(&mut self, description: String) {
        // TODO: Properly generate id
        let task = Task {
            id: self.tasks.len(), done: false, description
        };
        self.tasks.insert(task.id(), task);
    }
}

impl Task {
    pub fn id(&self) -> TaskID {
        self.id
    }
}
