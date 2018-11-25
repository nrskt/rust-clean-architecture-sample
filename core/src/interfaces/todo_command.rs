use crate::domain::Todo;

pub trait IsTodoCommand {
    fn save(&self, todo: Todo) -> Result<(), ()>;
}

pub trait HaveTodoCommand {
    type T: IsTodoCommand;
    fn provide(&self) -> &Self::T;
}

pub trait TodoCommand {}
