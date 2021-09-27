use chrono::{DateTime, Local};
use crate::{
    db::DbConnection,
    sql_utils::value,
};
pub struct Artist {
    id: i64,
    name: String,
    bio: String,
    active: bool,
    createddate: DateTime<Local>,
    lasteditdate: DateTime<Local>,
}
impl Artist {
    fn get_by_row(row: &rusqlite::Row) -> Result<Self, rusqlite::Error> {
        let id = value(row, "Id")?;
        let name = value(row, "Name")?;
        let bio = value(row, "Bio")?;
        let active = value(row, "Active")?;
        let createddate = value(row, "CreatedDate")?;
        let lasteditdate = value(row, "LastEditDate")?;
        Ok(Self { id, name, bio, active, createddate, lasteditdate })
    }
    pub fn insert_new<'a>(db: &mut DbConnection, name: &'a str, bio: &'a str, active: bool) -> Result<Self, rusqlite::Error> {
        const INSERT_NEW_SQL: &'static str = include_str!("./sql/insert_new.sql");
        let new_id;
        {
            let c = db.get_connection();
            let mut tx = c.transaction()?;
            let sp = tx.savepoint()?;
            let mut stmt = sp.prepare(INSERT_NEW_SQL)?;
            new_id = stmt.insert(rusqlite::named_params!{ ":name": name, ":bio": bio, ":active": active })?;
        }
        return Self::get_by_id(db, new_id);
    }
    pub fn get_all(db: &mut DbConnection) -> Result<Vec<Self>, rusqlite::Error> {
        const GET_ALL_SQL: &'static str = include_str!("./sql/get_all.sql");
        let c = db.get_connection();
        let mut stmt = c.prepare(GET_ALL_SQL)?;
        let artists = stmt.query_map([], |row| {
            Self::get_by_row(&row)
        })?.into_iter().collect();
        return artists;
    }
    pub fn get_by_id<'a>(db: &mut DbConnection, id: i64) -> Result<Self, rusqlite::Error> {
        const GET_BY_ID_SQL: &'static str = include_str!("./sql/get_by_id.sql");
        let c = db.get_connection();
        let mut stmt = c.prepare(GET_BY_ID_SQL)?;
        return stmt.query_row(&[(":id", &id)], |row| {
            Self::get_by_row(&row)
        });
    }
    pub fn get_by_name<'a>(db: &mut DbConnection, name: &'a str) -> Result<Self, rusqlite::Error> {
        const GET_BY_NAME_SQL: &'static str = include_str!("./sql/get_by_name.sql");
        let c = db.get_connection();
        let mut stmt = c.prepare(GET_BY_NAME_SQL)?;
        return stmt.query_row(&[(":name", name)], |row| {
            Self::get_by_row(&row)
        });
    }
}