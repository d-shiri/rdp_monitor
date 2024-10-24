use mysql::Row;

#[derive(Debug)]
pub struct RDPData {
    pub full_name: String,
    pub remote_pc: String,
}
#[allow(dead_code)]
impl RDPData {
    pub fn from_row(row: Row) -> Self {
        Self {
            full_name: row.get("full_name").unwrap(),
            remote_pc: row.get("remote_pc").unwrap(),
        }
    }
}
