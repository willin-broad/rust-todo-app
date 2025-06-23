use crate::todo::TodoList;
use std::fs;
use std::io;
use std::path::Path;

const DATA_FILE: &str = "data/todos.json";

pub fn load_todos() -> Result<TodoList, Box<dyn std::error::Error>> {
    // Create data directory if it doesn't exist
    if let Some(parent) = Path::new(DATA_FILE).parent() {
        fs::create_dir_all(parent)?;
    }

    // Try to read the file
    match fs::read_to_string(DATA_FILE) {
        Ok(contents) => {
            if contents.trim().is_empty() {
                Ok(TodoList::new())
            } else {
                let mut todo_list: TodoList = serde_json::from_str(&contents)?;
                // Always recalculate next_id based on the highest current ID
                let max_id = todo_list.todos.iter().map(|t| t.id).max().unwrap_or(0);
                todo_list.next_id = max_id.saturating_add(1);
                Ok(todo_list)
            }
        }
        Err(ref e) if e.kind() == io::ErrorKind::NotFound => {
            // File doesn't exist, return empty todo list
            Ok(TodoList::new())
        }
        Err(e) => Err(Box::new(e)),
    }
}

pub fn save_todos(todo_list: &TodoList) -> Result<(), Box<dyn std::error::Error>> {
    // Create data directory if it doesn't exist
    if let Some(parent) = Path::new(DATA_FILE).parent() {
        fs::create_dir_all(parent)?;
    }

    let json = serde_json::to_string_pretty(todo_list)?;
    fs::write(DATA_FILE, json)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_save_and_load_todos() {
        let test_file = "test_todos.json";

        // Create a test todo list
        let mut todo_list = TodoList::new();
        todo_list.add_todo("Test task".to_string());

        // Save it
        let json = serde_json::to_string_pretty(&todo_list).unwrap();
        fs::write(test_file, json).unwrap();

        // Load it back
        let contents = fs::read_to_string(test_file).unwrap();
        let loaded_list: TodoList = serde_json::from_str(&contents).unwrap();

        assert_eq!(loaded_list.todos.len(), 1);
        assert_eq!(loaded_list.todos[0].title, "Test task");

        // Clean up
        fs::remove_file(test_file).unwrap();
    }
}
