use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginationContainer<T> {
    pub items: Vec<T>,
    pub done: bool,
}

impl<T> PaginationContainer<T> {
    pub fn new(items: Vec<T>, limit: i64) -> Self {
        let len = items.len();
        Self {
            items,
            done: len < (limit as usize + 1),
        }
    }
}
