use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaginationLimits<T> {
    pub offset: i64,
    pub limit: i64,
    pub opts: T,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpacePaginationOptions {
    pub asc: bool,
}
