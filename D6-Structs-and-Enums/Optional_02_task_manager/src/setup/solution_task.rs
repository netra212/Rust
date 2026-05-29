// * 🧪 RUSTFLAGS="--cfg solution_exists" cargo test --features solution

// TODO 1: Define Priority enum with variants: Low, Medium, High, Urgent
// Look up enum definition in Rust documentation
#[derive(Debug, Clone, PartialEq)]
pub enum Priority {
    Low,
    Medium,
    High,
    Urgent,
}

// TODO 2: Define Status enum with variants: Todo, InProgress, Review, Done
// Look up enum definition in Rust documentation
#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Todo,
    InProgress,
    Review,
    Done,
}

// TODO 3: Define Task struct with the following fields:
// - title: String
// - description: String
// - priority: Priority
// - status: Status
// - assignee: Option<String>
// Look up struct definition and Option<T> in Rust documentation
#[derive(Debug, Clone)]
pub struct Task {
    pub title: String,
    pub description: String,
    pub priority: Priority,
    pub status: Status,
    pub assignee: Option<String>,
}

impl Task {
    // TODO 4: Implement new() function to create a new Task
    // Parameters: title, description, priority
    // Default status should be Todo, assignee should be None
    // Look up struct initialization in Rust documentation
    pub fn new(title: &str, description: &str, priority: Priority) -> Self {
        Self {
            title: title.to_string(),
            description: description.to_string(),
            priority,
            status: Status::Todo,
            assignee: None,
        }
    }

    // TODO 5: Implement with_assignee() function
    // Same as new() but also takes an assignee name
    // Look up Option::Some() in Rust documentation
    pub fn with_assignee(
        title: &str,
        description: &str,
        priority: Priority,
        assignee: &str,
    ) -> Self {
        Self {
            title: title.to_string(),
            description: description.to_string(),
            priority,
            status: Status::Todo,
            assignee: Some(assignee.to_string()),
        }
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn get_priority(&self) -> &Priority {
        &self.priority
    }

    pub fn get_status(&self) -> &Status {
        &self.status
    }

    pub fn get_assignee(&self) -> &Option<String> {
        &self.assignee
    }

    // TODO 6: Implement mark_in_progress() method
    // Change the status field to Status::InProgress
    // This needs &mut self because we're modifying the task
    pub fn mark_in_progress(&mut self) {
        self.status = Status::InProgress;
    }

    // TODO 7: Implement mark_done() method
    // Change the status field to Status::Done
    pub fn mark_done(&mut self) {
        self.status = Status::Done;
    }

    // TODO 8: Implement is_completed() method
    // Return true if status is Done, false otherwise
    // Look up pattern matching with enums in Rust documentation
    pub fn is_completed(&self) -> bool {
        matches!(self.status, Status::Done)
    }

    // TODO 9: Implement priority_level() method
    // Return a number representing priority: Low=1, Medium=2, High=3, Urgent=4
    // Use pattern matching on the priority enum
    pub fn priority_level(&self) -> u8 {
        match self.priority {
            Priority::Low => 1,
            Priority::Medium => 2,
            Priority::High => 3,
            Priority::Urgent => 4,
        }
    }
}
