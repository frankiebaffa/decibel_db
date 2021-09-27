use chrono::{DateTime, Local};
use crate::sql_utils::value;
use rusqlite::Connection;
pub struct Album {
    id: i64,
    albumtype_id: i64,
    name: String,
    blurb: String,
    active: bool,
    createddate: DateTime<Local>,
    lasteditdate: DateTime<Local>,
}
const GET_ALL_SQL: &'static str = include_str!("./sql/get_all.sql");
impl Album {
    fn from_row(row: &rusqlite::Row) -> Result<Self, rusqlite::Error> {
            let id = value(row, "Id")?;
            let albumtype_id = value(row, "AlbumType_Id")?;
            let name = value(row, "Name")?;
            let blurb = value(row, "Blurb")?;
            let active = value(row, "Active")?;
            let createddate = value(row, "CreatedDate")?;
            let lasteditdate = value(row, "LastEditDate")?;
            Ok(Self { id, albumtype_id, name, blurb, active, createddate, lasteditdate })
    }
    pub fn get_all(c: &mut Connection) -> Result<Vec<Self>, rusqlite::Error> {
        let mut stmt = c.prepare(GET_ALL_SQL)?;
        let albums = stmt.query_map([], |row| {
            return Self::from_row(&row);
        })?.into_iter().collect();
        return albums;
    }
}
