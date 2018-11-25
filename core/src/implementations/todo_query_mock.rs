use crate::domain::Todo;
use crate::interfaces::todo_query::*;
use crate::types::*;
use crate::Mock;

impl IsTodoQuery for &Mock {
    fn get_todos(&self, statuses: Box<[TodoStatus]>) -> Result<Vec<Todo>, ()> {
        let collection = self.collection.clone();
        let collection = collection.try_lock().unwrap();
        let todos: Vec<Todo> = collection
            .clone()
            .into_iter()
            .filter(|item| statuses.contains(&item.status))
            .collect();
        Ok(todos)
    }

    fn get_todo_by_id(&self, todo_id: TodoId) -> Result<Option<Todo>, ()> {
        let collection = self.collection.clone();
        let collection = collection.try_lock().unwrap();
        let todos: Vec<Todo> = collection
            .clone()
            .into_iter()
            .filter(|item| item.id == todo_id)
            .collect();
        if todos.len() == 1 {
            let todo = &todos.clone()[0];
            return Ok(Some(todo.clone()));
        }
        return Ok(None);
    }
}

impl HaveTodoQuery for &Mock {
    type T = Self;
    fn provide(&self) -> &Self::T {
        &self
    }
}
