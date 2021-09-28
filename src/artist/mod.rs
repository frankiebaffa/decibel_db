use chrono::{DateTime, Local};
use crate::{
    db::traits::{
        dbmodel::DbModel,
        primarykey::{
            PrimaryKey,
            PrimaryKeyModel,
        },
        uniquename::UniqueName,
    },
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
impl DbModel for Artist {
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
impl PrimaryKey for Artist {
    fn get_by_id_sql() -> &'static str {
        return include_str!("./sql/get_by_id.sql");
    }
    fn get_id(&self) -> i64 {
        return self.id;
    }
}
impl UniqueName for Artist {
    fn get_by_name_sql() -> &'static str {
        return include_str!("./sql/get_by_name.sql");
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
}
