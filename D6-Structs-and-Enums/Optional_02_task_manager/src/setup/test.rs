#[cfg(feature = "solution")]
#[path = "solution_task.rs"]
mod solution_task;

#[cfg(feature = "solution")]
#[path = "solution_project.rs"]
mod solution_project;

#[cfg(feature = "solution")]
use solution_task::{Priority, Status, Task};
#[cfg(feature = "solution")]
use solution_project::Project;

#[cfg(not(feature = "solution"))]
use crate::task::{Priority, Status, Task};
#[cfg(not(feature = "solution"))]
use crate::project::Project;

// === Task Tests ===

#[test]
fn test_task_creation() {
    let task = Task::new("Test task", "Description", Priority::High);
    assert_eq!(task.get_title(), "Test task");
    assert_eq!(task.get_description(), "Description");
    assert_eq!(task.get_priority(), &Priority::High);
    assert_eq!(task.get_status(), &Status::Todo);
    assert!(task.get_assignee().is_none());
}

#[test]
fn test_task_with_assignee() {
    let task = Task::with_assignee("Test task", "Description", Priority::Medium, "Alice");
    assert_eq!(task.get_title(), "Test task");
    assert_eq!(task.get_assignee(), &Some("Alice".to_string()));
}

#[test]
fn test_task_mark_in_progress() {
    let mut task = Task::new("Test", "Desc", Priority::Low);
    assert_eq!(task.get_status(), &Status::Todo);
    task.mark_in_progress();
    assert_eq!(task.get_status(), &Status::InProgress);
}

#[test]
fn test_task_mark_done() {
    let mut task = Task::new("Test", "Desc", Priority::Low);
    task.mark_done();
    assert_eq!(task.get_status(), &Status::Done);
}

#[test]
fn test_task_is_completed() {
    let mut task = Task::new("Test", "Desc", Priority::Low);
    assert!(!task.is_completed());
    task.mark_done();
    assert!(task.is_completed());
}

#[test]
fn test_task_priority_level() {
    let low = Task::new("Test", "Desc", Priority::Low);
    let medium = Task::new("Test", "Desc", Priority::Medium);
    let high = Task::new("Test", "Desc", Priority::High);
    let urgent = Task::new("Test", "Desc", Priority::Urgent);

    assert_eq!(low.priority_level(), 1);
    assert_eq!(medium.priority_level(), 2);
    assert_eq!(high.priority_level(), 3);
    assert_eq!(urgent.priority_level(), 4);
}

// === Project Tests ===

#[test]
fn test_project_creation() {
    let project = Project::new("Test Project");
    assert_eq!(project.name, "Test Project");
    assert_eq!(project.get_all_tasks().len(), 0);
}

#[test]
fn test_project_add_task() {
    let mut project = Project::new("Test");
    project.add_task(Task::new("Task 1", "Desc", Priority::Low));
    project.add_task(Task::new("Task 2", "Desc", Priority::High));
    assert_eq!(project.get_all_tasks().len(), 2);
}

#[test]
fn test_project_get_tasks_by_status() {
    let mut project = Project::new("Test");
    project.add_task(Task::new("Task 1", "Desc", Priority::Low));
    let mut task2 = Task::new("Task 2", "Desc", Priority::High);
    task2.mark_done();
    project.add_task(task2);

    let todo_tasks = project.get_tasks_by_status(&Status::Todo);
    assert_eq!(todo_tasks.len(), 1);
    assert_eq!(todo_tasks[0].get_title(), "Task 1");

    let done_tasks = project.get_tasks_by_status(&Status::Done);
    assert_eq!(done_tasks.len(), 1);
    assert_eq!(done_tasks[0].get_title(), "Task 2");
}

#[test]
fn test_project_get_tasks_by_priority() {
    let mut project = Project::new("Test");
    project.add_task(Task::new("Task 1", "Desc", Priority::Low));
    project.add_task(Task::new("Task 2", "Desc", Priority::High));
    project.add_task(Task::new("Task 3", "Desc", Priority::High));

    let high_tasks = project.get_tasks_by_priority(&Priority::High);
    assert_eq!(high_tasks.len(), 2);

    let low_tasks = project.get_tasks_by_priority(&Priority::Low);
    assert_eq!(low_tasks.len(), 1);
}

#[test]
fn test_project_get_high_priority_incomplete() {
    let mut project = Project::new("Test");
    project.add_task(Task::new("Task 1", "Desc", Priority::Low));
    project.add_task(Task::new("Task 2", "Desc", Priority::High));
    project.add_task(Task::new("Task 3", "Desc", Priority::Urgent));

    let mut done_task = Task::new("Task 4", "Desc", Priority::High);
    done_task.mark_done();
    project.add_task(done_task);

    let high_incomplete = project.get_high_priority_incomplete();
    assert_eq!(high_incomplete.len(), 2);
}

#[test]
fn test_project_count_by_status() {
    let mut project = Project::new("Test");
    project.add_task(Task::new("Task 1", "Desc", Priority::Low));
    project.add_task(Task::new("Task 2", "Desc", Priority::Low));

    let mut task3 = Task::new("Task 3", "Desc", Priority::Low);
    task3.mark_done();
    project.add_task(task3);

    let (todo, in_progress, review, done) = project.count_by_status();
    assert_eq!(todo, 2);
    assert_eq!(in_progress, 0);
    assert_eq!(review, 0);
    assert_eq!(done, 1);
}

#[test]
fn test_project_mark_task_done() {
    let mut project = Project::new("Test");
    project.add_task(Task::new("Task 1", "Desc", Priority::Low));
    project.add_task(Task::new("Task 2", "Desc", Priority::High));

    let result = project.mark_task_done("Task 1");
    assert!(result);

    let done_tasks = project.get_tasks_by_status(&Status::Done);
    assert_eq!(done_tasks.len(), 1);
    assert_eq!(done_tasks[0].get_title(), "Task 1");

    let not_found = project.mark_task_done("Task 99");
    assert!(!not_found);
}

#[test]
fn test_project_get_assignee_tasks() {
    let mut project = Project::new("Test");
    project.add_task(Task::with_assignee("Task 1", "Desc", Priority::Low, "Alice"));
    project.add_task(Task::with_assignee("Task 2", "Desc", Priority::High, "Bob"));
    project.add_task(Task::with_assignee("Task 3", "Desc", Priority::Medium, "Alice"));
    project.add_task(Task::new("Task 4", "Desc", Priority::Low));

    let alice_tasks = project.get_assignee_tasks("Alice");
    assert_eq!(alice_tasks.len(), 2);

    let bob_tasks = project.get_assignee_tasks("Bob");
    assert_eq!(bob_tasks.len(), 1);

    let charlie_tasks = project.get_assignee_tasks("Charlie");
    assert_eq!(charlie_tasks.len(), 0);
}

#[test]
fn test_project_completion_percentage() {
    let mut project = Project::new("Test");
    assert_eq!(project.completion_percentage(), 0.0);

    project.add_task(Task::new("Task 1", "Desc", Priority::Low));
    project.add_task(Task::new("Task 2", "Desc", Priority::Low));
    project.add_task(Task::new("Task 3", "Desc", Priority::Low));
    project.add_task(Task::new("Task 4", "Desc", Priority::Low));
    assert_eq!(project.completion_percentage(), 0.0);

    project.mark_task_done("Task 1");
    assert_eq!(project.completion_percentage(), 25.0);

    project.mark_task_done("Task 2");
    assert_eq!(project.completion_percentage(), 50.0);

    project.mark_task_done("Task 3");
    project.mark_task_done("Task 4");
    assert_eq!(project.completion_percentage(), 100.0);
}

// === Source Code Pattern Tests ===

#[test]
fn test_priority_enum_variants() {
    let source_code = get_task_source();
    assert!(source_code.contains("enum Priority"));
    assert!(source_code.contains("Low"));
    assert!(source_code.contains("Medium"));
    assert!(source_code.contains("High"));
    assert!(source_code.contains("Urgent"));
}

#[test]
fn test_status_enum_variants() {
    let source_code = get_task_source();
    assert!(source_code.contains("enum Status"));
    assert!(source_code.contains("Todo"));
    assert!(source_code.contains("InProgress"));
    assert!(source_code.contains("Review"));
    assert!(source_code.contains("Done"));
}

#[test]
fn test_task_struct_fields() {
    let source_code = get_task_source();
    assert!(source_code.contains("struct Task"));
    assert!(source_code.contains("pub title: String") || source_code.contains("pub title : String"));
    assert!(source_code.contains("pub description: String") || source_code.contains("pub description : String"));
    assert!(source_code.contains("pub priority: Priority") || source_code.contains("pub priority : Priority"));
    assert!(source_code.contains("pub status: Status") || source_code.contains("pub status : Status"));
    assert!(source_code.contains("pub assignee: Option<String>") || source_code.contains("pub assignee : Option<String>"));
}

#[test]
fn test_project_struct_fields() {
    let source_code = get_project_source();
    assert!(source_code.contains("struct Project"));
    assert!(source_code.contains("pub name: String") || source_code.contains("pub name : String"));
    assert!(source_code.contains("pub tasks: Vec<Task>") || source_code.contains("pub tasks : Vec<Task>"));
}

fn get_task_source() -> &'static str {
    #[cfg(all(feature = "solution", solution_exists))]
    {
        include_str!("solution_task.rs")
    }
    #[cfg(not(all(feature = "solution", solution_exists)))]
    {
        include_str!("../task.rs")
    }
}

fn get_project_source() -> &'static str {
    #[cfg(all(feature = "solution", solution_exists))]
    {
        include_str!("solution_project.rs")
    }
    #[cfg(not(all(feature = "solution", solution_exists)))]
    {
        include_str!("../project.rs")
    }
}
