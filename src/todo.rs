use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: usize,
    pub title: String,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl Todo {
    pub fn new(id: usize, title: String) -> Self {
        Todo {
            id,
            title,
            completed: false,
            created_at: Utc::now(),
            completed_at: None,
        }
    }

    pub fn complete(&mut self) {
        self.completed = true;
        self.completed_at = Some(Utc::now());
    }

    pub fn uncomplete(&mut self) {
        self.completed = false;
        self.completed_at = None;
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoList {
    pub todos: Vec<Todo>,
    pub next_id: usize,
}

impl TodoList {
    pub fn new() -> Self {
        TodoList {
            todos: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add_todo(&mut self, title: String) -> &Todo {
        let todo = Todo::new(self.next_id, title);
        self.next_id += 1;
        self.todos.push(todo);
        self.todos.last().unwrap()
    }

    #[allow(dead_code)]
    pub fn get_todo(&self, id: usize) -> Option<&Todo> {
        self.todos.iter().find(|todo| todo.id == id)
    }

    pub fn get_todo_mut(&mut self, id: usize) -> Option<&mut Todo> {
        self.todos.iter_mut().find(|todo| todo.id == id)
    }

    pub fn remove_todo(&mut self, id: usize) -> Option<Todo> {
        if let Some(pos) = self.todos.iter().position(|todo| todo.id == id) {
            Some(self.todos.remove(pos))
        } else {
            None
        }
    }

    pub fn list_todos(&self) -> &Vec<Todo> {
        &self.todos
    }

    pub fn list_pending(&self) -> Vec<&Todo> {
        self.todos.iter().filter(|todo| !todo.completed).collect()
    }

    pub fn list_completed(&self) -> Vec<&Todo> {
        self.todos.iter().filter(|todo| todo.completed).collect()
    }
}
