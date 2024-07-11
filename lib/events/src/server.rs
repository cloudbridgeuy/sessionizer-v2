use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct List {
    pub user_id: Option<u32>,
}
