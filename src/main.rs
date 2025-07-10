// fn main() {
//     println!("Hello, world!");
// }

mod storage;
mod todo;

use clap::{Parser, Subcommand};
use std::process;

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "A simple command-line todo application")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new todo item
    Add {
        /// The task description
        task: String,
    },
    /// List todo items
    List {
        /// Show only pending tasks
        #[arg(short, long)]
        pending: bool,
        /// Show only completed tasks
        #[arg(short, long)]
        completed: bool,
    },
    /// Mark a todo item as complete
    Complete {
        /// The ID of the task to complete
        id: usize,
    },
    /// Mark a todo item as incomplete
    Uncomplete {
        /// The ID of the task to mark as incomplete
        id: usize,
    },
    /// Delete a todo item
    Delete {
        /// The ID of the task to delete
        id: usize,
    },
    ///export todos to file
    Export {
        /// output file path
        #[arg(short, long, default_value = "todos.csv")]
        output: String,
    },
}

fn main() {
    let cli = Cli::parse();

    // Load existing todos
    let mut todo_list = match storage::load_todos() {
        Ok(list) => list,
        Err(e) => {
            eprintln!("Error loading todos: {}", e);
            process::exit(1);
        }
    };

    // Execute the command
    let result = match cli.command {
        Commands::Add { task } => add_todo(&mut todo_list, task),
        Commands::List { pending, completed } => list_todos(&todo_list, pending, completed),
        Commands::Complete { id } => complete_todo(&mut todo_list, id),
        Commands::Uncomplete { id } => uncomplete_todo(&mut todo_list, id),
        Commands::Delete { id } => delete_todo(&mut todo_list, id),
        Commands::Export { output } => export_todos(&todo_list, &output),
    };

    // Handle the result
    if let Err(e) = result {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn add_todo(
    todo_list: &mut todo::TodoList,
    task: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let todo_id;
    let todo_title;
    {
        let todo = todo_list.add_todo(task);
        todo_id = todo.id;
        todo_title = todo.title.clone();
    } // mutable borrow ends here

    storage::save_todos(todo_list)?;
    println!("Added todo #{}: {}", todo_id, todo_title);
    Ok(())
}

fn list_todos(
    todo_list: &todo::TodoList,
    pending: bool,
    completed: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let todos: Vec<&todo::Todo> = if pending {
        todo_list.list_pending()
    } else if completed {
        todo_list.list_completed()
    } else {
        todo_list.list_todos().iter().collect()
    };

    if todos.is_empty() {
        println!("No todos found.");
        return Ok(());
    }

    println!("Your todos:");
    println!("{:-<60}", "");

    let total = todos.len();
    for todo in todos {
        let status = if todo.completed { "✓" } else { " " };
        let title = if todo.completed {
            format!("\x1b[90m{}\x1b[0m", todo.title) // Gray text for completed
        } else {
            todo.title.clone()
        };

        println!("[{}] #{}: {}", status, todo.id, title);

        if todo.completed {
            if let Some(completed_at) = todo.completed_at {
                println!("    Completed at: {}", completed_at);
            }
        }
    }

    println!("{:-<60}", "");
    println!("Total: {} todos", total);
    Ok(())
}

fn complete_todo(
    todo_list: &mut todo::TodoList,
    id: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let todo_title;
    {
        let todo = todo_list.get_todo_mut(id).ok_or("Todo not found")?;
        todo.complete();
        todo_title = todo.title.clone();
    } // mutable borrow ends here

    storage::save_todos(todo_list)?;
    println!("Completed todo #{}: {}", id, todo_title);
    Ok(())
}

fn uncomplete_todo(
    todo_list: &mut todo::TodoList,
    id: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let todo_title;
    {
        let todo = todo_list.get_todo_mut(id).ok_or("Todo not found")?;
        todo.uncomplete();
        todo_title = todo.title.clone();
    } // mutable borrow ends here

    storage::save_todos(todo_list)?;
    println!("Marked todo #{} as pending: {}", id, todo_title);
    Ok(())
}

fn delete_todo(
    todo_list: &mut todo::TodoList,
    id: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    match todo_list.remove_todo(id) {
        Some(todo) => {
            storage::save_todos(todo_list)?;
            println!("Deleted todo #{}: {}", id, todo.title);
            Ok(())
        }
        None => Err(format!("Todo with ID {} not found.", id).into()),
    }
}
fn export_todos(
    todo_list: &todo::TodoList,
    output: &str,
) -> Result<(), Box<dyn std::error::Error>> {
   let mut wtr = csv::Writer::from_path(output)?;
   wtr.write_record(&["ID", "Title", "completed","created_at", "Completed At"])?;

    for todo in todo_list.list_todos() {
        wtr.write_record(&[
            todo.id.to_string(),
            todo.title.clone(),
            todo.completed.to_string(),
            todo.completed_at.map(|dt|dt.to_rfc3339()).unwrap_or_default(),
            todo.completed_at.map(|dt| dt.to_rfc3339()).unwrap_or_default(),

        ])?;
    }
    wtr.flush()?;
    println!("Todos exported to {}", output);
    Ok(())
}
