use crate::domain::Todo;
use crate::types::*;

pub trait IsTodoQuery {
    fn get_todos(&self, statuses: Box<[TodoStatus]>) -> Result<Vec<Todo>, ()>;
    fn get_todo_by_id(&self, todo_id: TodoId) -> Result<Option<Todo>, ()>;
}

pub trait HaveTodoQuery {
    type T: IsTodoQuery;
    fn provide(&self) -> &Self::T;
}

pub trait TodoQuery {}
