use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct List {
    pub user_id: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Create {
    pub user_id: Option<u32>,
    pub name: String,
    pub recreate: bool,
}
