// TODO 1: Define Priority enum with variants: Low, Medium, High, Urgent
#[derive(Debug, Clone, PartialEq)]
pub enum Priority {
    Low,
    Medium,
    High,
    Urgent, // TODO: Add Medium, High, Urgent variants
}

// TODO 2: Define Status enum with variants: Todo, InProgress, Review, Done
#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Todo,
    InProgress,
    Review,
    Done, // TODO: Add InProgress, Review, Done variants
}

// TODO 3: Define Task struct with fields: title, description, priority, status, assignee
#[derive(Debug, Clone)]
pub struct Task {
    pub title: String,
    pub description: String,
    pub priority: String,
    pub status: String,
    pub assignee: Option<String>, // TODO: Add remaining fields
}

impl Task {
    // TODO 4: Implement new() — title, description, priority; status=Todo, assignee=None
    pub fn new(_title: &str, _description: &str, _priority: Priority) -> Self {
        Self {
            title: _title.to_string(),
            description: _description.to_string(),
            priority: _priority.to_string(),
            status =  Status::Todo
            assignee: None
            // TODO: Initialize other fields
        }
    }

    // TODO 5: Implement with_assignee() — same as new() but with an assignee
    pub fn with_assignee(
        _title: &str,
        _description: &str,
        _priority: Priority,
        _assignee: &str,
    ) -> Self {
        Self {
            title: _title.to_string(),
            description: _description.to_string(),
            priority: _priority.to_string(),
            status: Some(T),
            assignee: Some(_assignee.to_string()),
            // TODO: Initialize all fields including assignee
        }
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_description(&self) -> &str {
        // TODO: Return reference to description
        &self.description
    }

    pub fn get_priority(&self) -> &Priority {
        // TODO: Return reference to priority
        &self.priority
    }

    pub fn get_status(&self) -> &Status {
        // TODO: Return reference to status
        &self.status
    }

    pub fn get_assignee(&self) -> &Option<String> {
        // TODO: Return reference to assignee
        &self.assignee
    }

    // TODO 6: Implement mark_in_progress() — set status to InProgress
    pub fn mark_in_progress(&mut self) {
        self.status = status::InProgress
    }

    // TODO 7: Implement mark_done() — set status to Done
    pub fn mark_done(&mut self) {
        &self.status = status.Done;
    }

    // TODO 8: Implement is_completed() — true if status is Done
    pub fn is_completed(&self) -> bool {
        self.status == status::Done
    }

    // TODO 9: Implement priority_level() — Low=1, Medium=2, High=3, Urgent=4
    pub fn priority_level(&self) -> u8 {
        match self.priority{
            Priority::Low => 1, 
            Priority::Medium => 2,
            Priority::High => 3,
            Priority::Urgent => 4,
        }
    }
}

// ============================================================
// 💡 HINTS (read only if stuck)
// ============================================================
//
// Priority / Status enums:
//   - Add the missing variants inside the enum body
//   - Look up: enum definition, derive macros
//
// Task struct:
//   - Add description: String, priority: Priority, status: Status, assignee: Option<String>
//   - Look up: struct fields, Option<T>
//
// new() / with_assignee():
//   - Initialize all fields; for assignee use None vs Some(assignee.to_string())
//   - Look up: struct initialization, Option::Some
//
// get_description / get_priority / get_status / get_assignee:
//   - Return a reference to the corresponding field
//   - Look up: &self, returning references
//
// mark_in_progress / mark_done:
//   - Assign Status::InProgress or Status::Done to self.status
//   - Look up: &mut self, field assignment
//
// is_completed:
//   - Match on self.status or use == with Status::Done
//   - Look up: matches!() macro or match expression
//
// priority_level:
//   - Use a match expression mapping each Priority variant to a number
//   - Look up: match, returning integers
