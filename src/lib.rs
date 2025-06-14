pub mod storage;
pub mod todo;

#[cfg(test)]
mod tests {
    use super::todo::*;

    #[test]
    fn test_todo_creation() {
        let todo = Todo::new(1, "Test task".to_string());
        assert_eq!(todo.id, 1);
        assert_eq!(todo.title, "Test task");
        assert!(!todo.completed);
        assert!(todo.completed_at.is_none());
    }

    #[test]
    fn test_todo_completion() {
        let mut todo = Todo::new(1, "Test task".to_string());
        todo.complete();
        assert!(todo.completed);
        assert!(todo.completed_at.is_some());
    }

    #[test]
    fn test_todo_list_operations() {
        let mut todo_list = TodoList::new();

        // Test adding todos
        let todo1 = todo_list.add_todo("Task 1".to_string());
        assert_eq!(todo1.id, 1);
        assert_eq!(todo_list.todos.len(), 1);

        let todo2 = todo_list.add_todo("Task 2".to_string());
        assert_eq!(todo2.id, 2);
        assert_eq!(todo_list.todos.len(), 2);

        // Test getting todos
        assert!(todo_list.get_todo(1).is_some());
        assert!(todo_list.get_todo(3).is_none());

        // Test completing todo
        if let Some(todo) = todo_list.get_todo_mut(1) {
            todo.complete();
        }

        // Test filtering
        assert_eq!(todo_list.list_pending().len(), 1);
        assert_eq!(todo_list.list_completed().len(), 1);

        // Test removing todo
        let removed = todo_list.remove_todo(1);
        assert!(removed.is_some());
        assert_eq!(todo_list.todos.len(), 1);
    }
}
