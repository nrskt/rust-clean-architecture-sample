#[derive(Debug, PartialEq)]
pub enum TodoStatus {
    Todo,
    Done,
}

impl Eq for TodoStatus {}

pub fn all_todo_statuses() -> Box<[TodoStatus]> {
    Box::new([TodoStatus::Todo, TodoStatus::Done])
}
