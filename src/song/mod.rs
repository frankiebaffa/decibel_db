use chrono::{DateTime, Local};
use crate::{db::DbConnection, sql_utils::value};
struct Song {
    id: i64,
    name: String,
    blurb: String,
    active: bool,
    createddate: DateTime<Local>,
    lasteditdate: DateTime<Local>,
}
impl Song {

}
