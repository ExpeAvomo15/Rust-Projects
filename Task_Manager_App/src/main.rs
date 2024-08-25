use std::io;

// Represents a single task with a description
struct Task {
    description: String,
}

impl Task {
    // Creates a new Task from a description
    fn new(description: &str) -> Task {
        Task {
            description: description.to_string(),
        }
    }
}

// Manages a list of tasks
struct TaskList {
    tasks: Vec<Task>,
}

impl TaskList {
    // Creates a new, empty TaskList
    fn new() -> TaskList {
        TaskList {
            tasks: Vec::new(),
        }
    }

    // Adds a new task to the list
    fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    // Removes a task by index, if the index is valid
    fn remove_task(&mut self, index: usize) -> Result<(), String> {
        if index < self.tasks.len() {
            self.tasks.remove(index);
            Ok(())
        } else {
            Err("Index exceeds the length of the task list.".to_string())
        }
    }

    // Lists all tasks with their indices
    fn list_tasks(&self) {
        for (i, task) in self.tasks.iter().enumerate() {
            println!("{}. {}", i + 1, task.description);
        }
    }
}

// Displays the menu options to the user
fn show_menu() {
    println!("Select an option:");
    println!("1. Add Task");
    println!("2. Remove Task");
    println!("3. List Tasks");
    println!("4. Exit");
}

// Reads user input from the command line
fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()  // Return the trimmed input as a new String
}

fn main() {
    let mut task_list = TaskList::new();  // Ownership of the TaskList is here

    println!("Welcome to the Rust Task Manager App");

    loop {
        show_menu();
        let option = read_input();

        match option.as_str() {
            "1" => { // Add Task
                println!("Enter task description: ");
                let description = read_input();
                let task = Task::new(&description);  // Ownership of task is passed to TaskList
                task_list.add_task(task);
                println!("Task added successfully.");
            }
            "2" => { // Remove Task
                println!("Enter the task number to remove: ");
                let index: usize = read_input().parse().unwrap_or(0);
                match task_list.remove_task(index - 1) {
                    Ok(()) => println!("Task removed successfully."),
                    Err(err) => println!("Error: {}", err),
                }
            }
            "3" => { // List Tasks
                task_list.list_tasks();
            }
            "4" => { // Exit
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid option. Please try again.");
            }
        }
    }
}
