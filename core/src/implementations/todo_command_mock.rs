use crate::domain::Todo;
use crate::interfaces::todo_command::*;
use crate::Mock;

impl IsTodoCommand for &Mock {
    fn save(&self, todo: Todo) -> Result<(), ()> {
        let collection = self.collection.clone();
        let mut collection = collection.try_lock().unwrap();
        collection.push(todo);
        Ok(())
    }
}

impl HaveTodoCommand for &Mock {
    type T = Self;
    fn provide(&self) -> &Self::T {
        &self
    }
}
