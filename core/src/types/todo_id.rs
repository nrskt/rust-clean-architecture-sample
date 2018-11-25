use uuid::Uuid;

#[derive(Debug)]
pub struct TodoId(Uuid);

impl TodoId {
    pub fn new() -> TodoId {
        TodoId(Uuid::new_v4())
    }
}

impl PartialEq for TodoId {
    fn eq(&self, other: &TodoId) -> bool {
        let TodoId(a) = self;
        let TodoId(b) = other;
        a == b
    }
}

impl Eq for TodoId {}

#[test]
fn test_eq() {
    let a = TodoId::new();
    let b = TodoId::new();
    assert_ne!(a, b)
}
