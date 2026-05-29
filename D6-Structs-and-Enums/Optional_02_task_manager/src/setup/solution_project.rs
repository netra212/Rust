// * 🧪 RUSTFLAGS="--cfg solution_exists" cargo test --features solution

use super::solution_task::{Priority, Status, Task};

// TODO 1: Define Project struct with the following fields:
// - name: String
// - tasks: Vec<Task>
// Look up Vec in Rust documentation
#[derive(Debug)]
pub struct Project {
    pub name: String,
    pub tasks: Vec<Task>,
}

impl Project {
    // TODO 2: Implement new() function to create an empty project
    // Initialize tasks as an empty Vec
    // Look up Vec::new() in Rust documentation
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            tasks: Vec::new(),
        }
    }

    // TODO 3: Implement add_task() method
    // Add a task to the tasks Vec
    // Look up Vec push method in Rust documentation
    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    // TODO 4: Implement get_tasks_by_status() method
    // Return a Vec of references to tasks that match the given status
    // Use filter() and collect()
    // Look up iterator methods in Rust documentation
    pub fn get_tasks_by_status(&self, status: &Status) -> Vec<&Task> {
        self.tasks
            .iter()
            .filter(|task| task.get_status() == status)
            .collect()
    }

    // TODO 5: Implement get_tasks_by_priority() method
    // Return a Vec of references to tasks that match the given priority
    // Use filter() and collect()
    pub fn get_tasks_by_priority(&self, priority: &Priority) -> Vec<&Task> {
        self.tasks
            .iter()
            .filter(|task| task.get_priority() == priority)
            .collect()
    }

    // TODO 6: Implement get_high_priority_incomplete() method
    // Return tasks that are High or Urgent priority AND not Done
    // Use filter() with multiple conditions
    // Look up pattern matching in filter predicates
    pub fn get_high_priority_incomplete(&self) -> Vec<&Task> {
        self.tasks
            .iter()
            .filter(|task| {
                matches!(task.get_priority(), Priority::High | Priority::Urgent)
                    && !task.is_completed()
            })
            .collect()
    }

    // TODO 7: Implement count_by_status() method
    // Count tasks in each status and return as tuple: (todo, in_progress, review, done)
    // Use filter() and len() for each status
    pub fn count_by_status(&self) -> (usize, usize, usize, usize) {
        let todo = self
            .tasks
            .iter()
            .filter(|task| matches!(task.get_status(), Status::Todo))
            .count();
        let in_progress = self
            .tasks
            .iter()
            .filter(|task| matches!(task.get_status(), Status::InProgress))
            .count();
        let review = self
            .tasks
            .iter()
            .filter(|task| matches!(task.get_status(), Status::Review))
            .count();
        let done = self
            .tasks
            .iter()
            .filter(|task| matches!(task.get_status(), Status::Done))
            .count();

        (todo, in_progress, review, done)
    }

    // TODO 8: Implement mark_task_done() method
    // Find a task by title and mark it as done
    // Return true if task was found and updated, false otherwise
    // Use iter_mut() to get mutable references
    // Look up find() method in Rust documentation
    pub fn mark_task_done(&mut self, title: &str) -> bool {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.get_title() == title) {
            task.mark_done();
            true
        } else {
            false
        }
    }

    // TODO 9: Implement get_assignee_tasks() method
    // Return all tasks assigned to a specific person
    // Handle the Option<String> assignee field
    // Look up Option methods (as_ref, map) in Rust documentation
    pub fn get_assignee_tasks(&self, assignee: &str) -> Vec<&Task> {
        self.tasks
            .iter()
            .filter(|task| {
                if let Some(task_assignee) = task.get_assignee() {
                    task_assignee == assignee
                } else {
                    false
                }
            })
            .collect()
    }

    // TODO 10: Implement completion_percentage() method
    // Calculate percentage of tasks that are Done
    // Return 0.0 if there are no tasks
    // Formula: (done_count / total_count) * 100.0
    pub fn completion_percentage(&self) -> f64 {
        if self.tasks.is_empty() {
            return 0.0;
        }

        let done_count = self.tasks.iter().filter(|task| task.is_completed()).count();
        (done_count as f64 / self.tasks.len() as f64) * 100.0
    }

    pub fn get_all_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }
}
