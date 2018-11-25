use crate::domain::Todo;
use crate::types::*;
use uuid::Uuid;

pub fn mock_uuids() -> Vec<TodoId> {
    vec![
        Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap(),
        Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap(),
        Uuid::parse_str("00000000-0000-0000-0000-000000000003").unwrap(),
        Uuid::parse_str("00000000-0000-0000-0000-000000000004").unwrap(),
        Uuid::parse_str("00000000-0000-0000-0000-000000000005").unwrap(),
    ]
    .iter()
    .map(|uuid| TodoId::from_uuid(*uuid))
    .collect()
}

pub fn mock_todos() -> Vec<Todo> {
    let mut todos: Vec<Todo> = mock_uuids()
        .iter()
        .enumerate()
        .map(move |(i, todo_id)| {
            let todo_id = todo_id.clone();
            Todo::new(todo_id, format!("test_{}", i)).unwrap()
        })
        .collect();
    todos[0].done();
    todos
}
