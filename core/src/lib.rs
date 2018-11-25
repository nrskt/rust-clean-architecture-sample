use std::sync::{Arc, Mutex};

pub mod domain;
pub mod dto;
pub mod interfaces;
pub mod mock_tools;
pub mod types;
pub mod usecases;

mod implementations;

pub struct Mock {
    pub collection: Arc<Mutex<Vec<domain::Todo>>>,
}

#[test]
fn mock() {
    let m = Mock {
        collection: Arc::new(Mutex::new(vec![])),
    };
    let r = usecases::get_all_todos(&m);
    assert_eq!(r, Ok(vec![]));
    let _ = usecases::create_new_todo(&m, types::TodoId::new(), "add test".to_string());
    let r = usecases::get_all_todos(&m);
    match r {
        Ok(v) => {
            assert_eq!(v.len(), 1);
            assert_eq!(
                v[0].title,
                types::TodoTitle::new("add test".to_string()).unwrap()
            )
        }
        Err(_) => assert!(false),
    }
}
