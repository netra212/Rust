use crate::task::{Priority, Status, Task};

// TODO 1: Define Project struct with fields: name: String, tasks: Vec<Task>
#[derive(Debug)]
pub struct Project {
    pub name: String,
    pub tasks: Vec<Task>, // TODO: Add tasks field
}

impl Project {
    // TODO 2: Implement new() — create an empty project with an empty tasks Vec
    pub fn new(_name: &str) -> Self {
        Self {
            name: _name.to_string(),
            tasks: Vec::new(), // TODO: Initialize tasks
        }
    }

    // TODO 3: Implement add_task() — push a task into the tasks Vec
    pub fn add_task(&mut self, _task: Task) {
        // TODO: Add task to the Vec
        self.tasks.push(_task);
    }

    // TODO 4: Implement get_tasks_by_status() — return refs to tasks matching the status
    pub fn get_tasks_by_status(&self, _status: &Status) -> Vec<&Task> {
        self.tasks
            .iter()
            .filter(|t| t.get_status() == _status)
            .collect()
    }

    // TODO 5: Implement get_tasks_by_priority() — return refs to tasks matching the priority
    pub fn get_tasks_by_priority(&self, _priority: &Priority) -> Vec<&Task> {
        self.tasks
            .iter()
            .filter(|t| t.get_tasks_by_priority() == _priority)
            .collect()
    }

    // TODO 6: Implement get_high_priority_incomplete() — High or Urgent priority AND not Done
    pub fn get_high_priority_incomplete(&self) -> Vec<&Task> {
        self.tasks
            .iter()
            .filter(|t| {
                t.get_priority == &Priority::High
                    || t.get_priority() == &Priority::Urgent && &&t.get_status() != &Status::Done
            })
            .collect()
    }

    // TODO 7: Implement count_by_status() — (todo, in_progress, review, done) counts
    pub fn count_by_status(&self) -> (usize, usize, usize, usize) {
        (0, 0, 0, 0)
    }

    // TODO 8: Implement mark_task_done() — find by title and mark done; return true if found
    pub fn mark_task_done(&mut self, _title: &str) -> bool {
        false
    }

    // TODO 9: Implement get_assignee_tasks() — return tasks assigned to a specific person
    pub fn get_assignee_tasks(&self, _assignee: &str) -> Vec<&Task> {
        vec![]
    }

    // TODO 10: Implement completion_percentage() — (done / total) * 100.0, or 0.0 if empty
    pub fn completion_percentage(&self) -> f64 {
        0.0
    }

    pub fn get_all_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }
}

// ============================================================
// 💡 HINTS (read only if stuck)
// ============================================================
//
// Project struct / new():
//   - Add tasks: Vec<Task> field; initialize with Vec::new()
//   - Look up: Vec, Vec::new()
//
// add_task:
//   - Use self.tasks.push(task)
//   - Look up: Vec::push()
//
// get_tasks_by_status / get_tasks_by_priority:
//   - self.tasks.iter().filter(|t| t.get_status() == status).collect()
//   - Look up: iter(), filter(), collect()
//
// get_high_priority_incomplete:
//   - Filter for tasks where priority is High or Urgent AND status is not Done
//   - Look up: matches!() macro, || and && operators in filter
//
// count_by_status:
//   - Call filter().count() for each status variant
//   - Look up: filter(), count()
//
// mark_task_done:
//   - Use iter_mut(), find the task by title, call mark_done() on it
//   - Look up: iter_mut(), find()
//
// get_assignee_tasks:
//   - Check task.get_assignee() with as_deref() or map() to compare &str
//   - Look up: Option::as_deref(), Option methods
//
// completion_percentage:
//   - Count done tasks, divide by total, multiply by 100.0
//   - Guard against division by zero when tasks is empty
//   - Look up: as f64, division
