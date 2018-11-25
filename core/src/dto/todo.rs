use crate::domain::Todo;
use crate::dto::to_domain::ToDomain;
use crate::types::*;

#[derive(Debug, PartialEq)]
pub struct TodoDto {
    id: TodoId,
    title: TodoTitle,
    status: TodoStatus,
}

impl Eq for TodoDto {}

impl From<Todo> for TodoDto {
    fn from(domain: Todo) -> Self {
        TodoDto {
            id: domain.id,
            title: domain.title,
            status: domain.status,
        }
    }
}

impl ToDomain<Todo> for TodoDto {
    fn to_domain(self) -> Result<Todo, ()> {
        Ok(Todo {
            id: self.id,
            title: self.title,
            status: self.status,
        })
    }
}

#[test]
fn test_todo_dto() {
    let id = TodoId::new();
    let todo = Todo::new(id, "test".to_string()).unwrap();
    let todo_c = todo.clone();
    let dto = TodoDto::from(todo);
    let revert = dto.to_domain().unwrap();
    assert_eq!(todo_c, revert)
}
