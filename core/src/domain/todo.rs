use crate::types::*;

pub struct Todo {
    _id: TodoId,
    _title: TodoTitle,
    status: TodoStatus,
}

impl Todo {
    pub fn new(id: TodoId, title: String) -> Result<Todo, ()> {
        let todo_title = TodoTitle::new(title)?;
        Ok(Todo {
            _id: id,
            _title: todo_title,
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
