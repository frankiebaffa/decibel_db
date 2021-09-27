use chrono::{DateTime, Local};
use crate::{
    db::DbModel,
    sql_utils::value,
};
use rusqlite::Connection;
pub struct Artist {
    id: i64,
    name: String,
    bio: String,
    active: bool,
    createddate: DateTime<Local>,
    lasteditdate: DateTime<Local>,
}
impl DbModel<Artist> for Artist {
    fn from_row(row: &rusqlite::Row) -> Result<Self, rusqlite::Error> {
        let id = value(row, "Id")?;
        let name = value(row, "Name")?;
        let bio = value(row, "Bio")?;
        let active = value(row, "Active")?;
        let createddate = value(row, "CreatedDate")?;
        let lasteditdate = value(row, "LastEditDate")?;
        Ok(Self { id, name, bio, active, createddate, lasteditdate })
    }
}
impl Artist {
    pub fn insert_new<'a>(c: &mut Connection, name: &'a str, bio: &'a str, active: bool) -> Result<Self, rusqlite::Error> {
        const INSERT_NEW_SQL: &'static str = include_str!("./sql/insert_new.sql");
        let new_id;
        {
            let mut tx = c.transaction()?;
            let sp = tx.savepoint()?;
            let mut stmt = sp.prepare(INSERT_NEW_SQL)?;
            new_id = stmt.insert(rusqlite::named_params!{ ":name": name, ":bio": bio, ":active": active })?;
        }
        return Self::get_by_id(c, new_id);
    }
    pub fn get_all(c: &mut Connection) -> Result<Vec<Self>, rusqlite::Error> {
        const GET_ALL_SQL: &'static str = include_str!("./sql/get_all.sql");
        let mut stmt = c.prepare(GET_ALL_SQL)?;
        let artists = stmt.query_map([], |row| {
            Self::from_row(&row)
        })?.into_iter().collect();
        return artists;
    }
    pub fn get_by_id<'a>(c: &mut Connection, id: i64) -> Result<Self, rusqlite::Error> {
        const GET_BY_ID_SQL: &'static str = include_str!("./sql/get_by_id.sql");
        let mut stmt = c.prepare(GET_BY_ID_SQL)?;
        return stmt.query_row(&[(":id", &id)], |row| {
            Self::from_row(&row)
        });
    }
    pub fn get_by_name<'a>(c: &mut Connection, name: &'a str) -> Result<Self, rusqlite::Error> {
        const GET_BY_NAME_SQL: &'static str = include_str!("./sql/get_by_name.sql");
        let mut stmt = c.prepare(GET_BY_NAME_SQL)?;
        return stmt.query_row(&[(":name", name)], |row| {
            Self::from_row(&row)
        });
    }
}
