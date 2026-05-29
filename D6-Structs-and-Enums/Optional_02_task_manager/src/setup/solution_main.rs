// * 🧪 RUSTFLAGS="--cfg solution_exists" cargo test --features solution

use super::solution_task::{Priority, Status, Task};
use super::solution_project::Project;

/* ========================================================== */
/*                      🦀 MAIN 🦀                            */
/* ========================================================== */

pub fn main() {
    let mut project = Project::new("Website Redesign");

    // Add tasks with different priorities
    project.add_task(Task::new(
        "Design homepage mockup",
        "Create wireframes and visual design",
        Priority::High,
    ));

    project.add_task(Task::with_assignee(
        "Set up database",
        "Configure PostgreSQL and create schema",
        Priority::Urgent,
        "Alice",
    ));

    project.add_task(Task::with_assignee(
        "Write documentation",
        "Document API endpoints and usage",
        Priority::Medium,
        "Bob",
    ));

    project.add_task(Task::new(
        "Update footer links",
        "Fix broken links in footer",
        Priority::Low,
    ));

    project.add_task(Task::with_assignee(
        "Implement authentication",
        "Add JWT-based auth system",
        Priority::High,
        "Alice",
    ));

    println!("=== Task Management System ===\n");
    println!("Project: {}\n", project.name);

    // Display all tasks
    println!("All Tasks:");
    for task in project.get_all_tasks() {
        let assignee = match task.get_assignee() {
            Some(name) => name.as_str(),
            None => "Unassigned",
        };
        println!(
            "  [{:?}] {:?} - {} (assigned to: {})",
            task.get_status(),
            task.get_priority(),
            task.get_title(),
            assignee
        );
    }

    // Show high priority incomplete tasks
    println!("\nHigh Priority Incomplete Tasks:");
    let high_priority = project.get_high_priority_incomplete();
    for task in &high_priority {
        println!("  - {}", task.get_title());
    }

    // Mark some tasks as done
    project.mark_task_done("Update footer links");
    project.mark_task_done("Design homepage mockup");

    // Show status breakdown
    let (todo, in_progress, review, done) = project.count_by_status();
    println!("\nTask Status Breakdown:");
    println!("  Todo: {}", todo);
    println!("  In Progress: {}", in_progress);
    println!("  Review: {}", review);
    println!("  Done: {}", done);

    // Show completion percentage
    println!(
        "\nCompletion: {:.1}%",
        project.completion_percentage()
    );

    // Show Alice's tasks
    println!("\nAlice's Tasks:");
    let alice_tasks = project.get_assignee_tasks("Alice");
    for task in alice_tasks {
        println!(
            "  [{:?}] {}",
            task.get_status(),
            task.get_title()
        );
    }
}
