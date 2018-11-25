use crate::domain::Todo;
use crate::interfaces::todo_command::*;
use crate::interfaces::todo_query::*;
use crate::types::*;

pub fn get_all_todos<T: HaveTodoQuery>(svc: T) -> Result<Vec<Todo>, ()> {
    let todo_query_svc = svc.provide();
    let r = todo_query_svc.get_todos(all_todo_statuses())?;
    Ok(r)
}

pub fn get_uncomplete_todos<T: HaveTodoQuery>(svc: T) -> Result<Vec<Todo>, ()> {
    let todo_query_svc = svc.provide();
    let r = todo_query_svc.get_todos(Box::new([TodoStatus::Todo]))?;
    Ok(r)
}

pub fn create_new_todo<T: HaveTodoCommand>(
    svc: T,
    todo_id: TodoId,
    title: String,
) -> Result<(), ()> {
    let todo_command_svc = svc.provide();
    let new_todo = Todo::new(todo_id, title)?;
    todo_command_svc.save(new_todo)
}

pub fn done_todo<T: HaveTodoQuery, U: HaveTodoCommand>(
    q_service: T,
    c_service: U,
    id: TodoId,
) -> Result<(), ()> {
    let query_svc = q_service.provide();
    let command_svc = c_service.provide();

    let todo = query_svc.get_todo_by_id(id)?;
    match todo {
        Some(mut todo) => {
            todo.done();
            command_svc.save(todo)
        }
        None => Err(()),
    }
}
