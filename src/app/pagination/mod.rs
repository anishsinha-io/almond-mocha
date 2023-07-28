use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginationContainer<T> {
    pub items: Vec<T>,
    pub done: bool,
    pub start: u64,
    pub end: u64,
}
