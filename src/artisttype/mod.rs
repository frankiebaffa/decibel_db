use chrono::{DateTime, Local};
use crate::{
    db::DbModel,
    sql_utils::value
};
use rusqlite::Connection;
pub struct ArtistType {
    id: i64,
    name: String,
    descriptor: String,
    description: String,
    active: bool,
    createddate: DateTime<Local>,
    lasteditdate: DateTime<Local>,
}
impl DbModel<ArtistType> for ArtistType {
    fn from_row(row: &rusqlite::Row) -> Result<Self, rusqlite::Error> {
        let id = value(row, "Id")?;
        let name = value(row, "Name")?;
        let descriptor = value(row, "Descriptor")?;
        let description = value(row, "Description")?;
        let active = value(row, "Active")?;
        let createddate = value(row, "CreatedDate")?;
        let lasteditdate = value(row, "LastEditDate")?;
        return Ok(ArtistType { id, name, descriptor, description, active, createddate, lasteditdate });
    }
}
impl ArtistType {
    pub const PERFORMER: &'static str = "Performer";
    pub const WRITER: &'static str = "Writer";
    pub const COMPOSER: &'static str = "Composer";
    pub const PRODUCER: &'static str = "Producer";
    pub const FEATURE: &'static str = "Feature";
    pub fn get_by_name<'a>(c: &mut Connection, name: &'a str) -> Result<Self, rusqlite::Error> {
        const GET_BY_NAME_SQL: &'static str = include_str!("./sql/get_by_name.sql");
        let mut stmt = c.prepare(GET_BY_NAME_SQL)?;
        return stmt.query_row(rusqlite::named_params!{ ":name": name }, |row| {
            Self::from_row(&row)
        });
    }
}
