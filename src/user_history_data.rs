use mysql::Row;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)] 
pub struct UserHistory {
    pub user_id: String,
    pub remote_pc: i32,
    pub rdp_start_time: String,
    pub rdp_end_time: String,
    //use std::fmt;
}
#[allow(dead_code)]
impl UserHistory {
    pub fn from_row(row: Row) -> Self {
        Self {
            user_id: row.get("user_id").unwrap(),
            remote_pc: row.get("remote_pc").unwrap(),
            rdp_start_time: row.get("rdp_start_time").unwrap(),
            rdp_end_time: row.get("rdp_end_time").unwrap(),
        }
    }
}
