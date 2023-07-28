use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaginationLimits {
    pub offset: u64,
    pub limit: u64,
}
