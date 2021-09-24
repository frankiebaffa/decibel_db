use chrono::{DateTime, Local};
use crate::{
    db::DbConnection,
    sql_utils::value,
};
pub struct AlbumType {
    id: i64,
    name: String,
    description: String,
    active: bool,
    createddate: DateTime<Local>,
    lasteditdate: DateTime<Local>,
}
impl AlbumType {

}
