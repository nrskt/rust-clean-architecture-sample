#[derive(Debug, Clone)]
pub struct TodoTitle(String);

impl TodoTitle {
    pub fn new(title: String) -> Result<TodoTitle, ()> {
        if title.len() > 20 {
            return Err(());
        }

        if title.len() == 0 {
            return Err(());
        }
        Ok(TodoTitle(title))
    }
}

impl PartialEq for TodoTitle {
    fn eq(&self, other: &TodoTitle) -> bool {
        let TodoTitle(a) = self;
        let TodoTitle(b) = other;
        a == b
    }
}

impl Eq for TodoTitle {}

#[test]
fn test_make_todo_title() {
    let title = "aaaaaaaaaa";
    let todo_title = TodoTitle::new(title.to_string());
    assert!(todo_title.is_ok());

    let title = "";
    let todo_title = TodoTitle::new(title.to_string());
    assert!(todo_title.is_err());

    let title = "テスト";
    let todo_title = TodoTitle::new(title.to_string());
    assert!(todo_title.is_ok());

    let title = "テストテストテストテストテストテストテスト";
    let todo_title = TodoTitle::new(title.to_string());
    assert!(todo_title.is_err());
}
