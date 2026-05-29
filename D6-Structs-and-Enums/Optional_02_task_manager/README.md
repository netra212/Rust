+++
title = "Optional 02 Task Manager"
description = "Build a task management system using structs, enums, and pattern matching"
+++

## Background & Objectives

Build a complete task management system that demonstrates advanced usage of structs and enums. This exercise combines multiple concepts: enum variants, pattern matching, Vec operations, and struct methods.

This exercise integrates concepts you've learned:
- Defining structs with multiple fields
- Creating enums with meaningful variants
- Implementing methods on structs
- Pattern matching with enums
- Working with Vec<T> to manage collections
- Using Option<T> for optional data
- Filtering and transforming collections

## Specs

### Project Structure

The project has three main files:
- `src/main.rs` - Main program demonstrating the task manager
- `src/task.rs` - Task struct and related enums (Priority, Status)
- `src/project.rs` - Project struct managing multiple tasks

### Your Tasks

#### In `src/task.rs`:

1. **TODO 1**: Define `Priority` enum with variants: Low, Medium, High, Urgent
2. **TODO 2**: Define `Status` enum with variants: Todo, InProgress, Review, Done
3. **TODO 3**: Define `Task` struct with fields:
   - title: String
   - description: String
   - priority: Priority
   - status: Status
   - assignee: Option<String>

4. **TODO 4**: Implement `Task::new()` constructor
5. **TODO 5**: Implement `Task::with_assignee()` - create task with optional assignee
6. **TODO 6**: Implement `Task::mark_in_progress()` - change status to InProgress
7. **TODO 7**: Implement `Task::mark_done()` - change status to Done
8. **TODO 8**: Implement `Task::is_completed()` - return true if status is Done
9. **TODO 9**: Implement `Task::priority_level()` - return numeric value (Low=1, Medium=2, High=3, Urgent=4)

#### In `src/project.rs`:

1. **TODO 1**: Define `Project` struct with fields:
   - name: String
   - tasks: Vec<Task>

2. **TODO 2**: Implement `Project::new()` to create empty project
3. **TODO 3**: Implement `Project::add_task()` to add task to collection
4. **TODO 4**: Implement `Project::get_tasks_by_status()` - filter tasks by status
5. **TODO 5**: Implement `Project::get_tasks_by_priority()` - filter tasks by priority
6. **TODO 6**: Implement `Project::get_high_priority_incomplete()` - get High/Urgent tasks not Done
7. **TODO 7**: Implement `Project::count_by_status()` - return tuple (todo, in_progress, review, done)
8. **TODO 8**: Implement `Project::mark_task_done()` - find task by title and mark as done
9. **TODO 9**: Implement `Project::get_assignee_tasks()` - get all tasks for specific assignee
10. **TODO 10**: Implement `Project::completion_percentage()` - calculate % of completed tasks

### How It Should Work

The interactive demo allows you to:
1. View all tasks with their status and priority
2. See tasks filtered by status (Todo, InProgress, etc.)
3. View high-priority incomplete tasks
4. Check completion statistics
5. View tasks by assignee

## Run & Test

### Run the Program

```bash
cargo run
```

### Test Your Solution

```bash
cargo test
```

## Key Concepts

- **Enums**: Creating meaningful variants (Priority, Status)
- **Structs**: Organizing related data (Task, Project)
- **Option<T>**: Handling optional fields (assignee)
- **Pattern Matching**: Matching enum variants for logic
- **Vec Operations**: filter, iter, collect for searching
- **Methods**: impl blocks for behavior (&self, &mut self)
- **Ownership**: Working with references and mutable state

## Learning Goals

This exercise teaches you to:
1. Design enums for state representation
2. Combine structs and enums effectively
3. Use pattern matching for enum-based logic
4. Manage collections of custom types
5. Implement filtering and aggregation methods
6. Handle optional data with Option<T>
7. Calculate derived values from collections
