use crate::types::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Todo {
    pub id: TodoId,
    pub title: TodoTitle,
    pub status: TodoStatus,
}

impl Eq for Todo {}

impl Todo {
    pub fn new(id: TodoId, title: String) -> Result<Todo, ()> {
        let todo_title = TodoTitle::new(title)?;
        Ok(Todo {
            id: id,
            title: todo_title,
            status: TodoStatus::Todo,
        })
    }

    pub fn done(&mut self) {
        self.status = TodoStatus::Done;
    }
}

#[test]
fn test_todo() {
    let todo_id = TodoId::new();
    let mut todo = Todo::new(todo_id, "test todo".to_string()).unwrap();
    assert_eq!(todo.status, TodoStatus::Todo);
    todo.done();
    assert_eq!(todo.status, TodoStatus::Done);
}
