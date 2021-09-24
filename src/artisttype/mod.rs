use chrono::{DateTime, Local};
use crate::{
    db::DbConnection,
    sql_utils::value,
};
pub struct ArtistType {
    id: i64,
    name: String,
    description: String,
    active: bool,
    createddate: DateTime<Local>,
    lasteditdate: DateTime<Local>,
}
impl ArtistType {

}
