use chrono::{DateTime, Local};
use crate::{
    db::DbConnection,
    sql_utils::value,
};
struct Artist {
    id: i64,
    name: String,
    bio: String,
    active: bool,
    createddate: DateTime<Local>,
    lasteditdate: DateTime<Local>,
}
const GET_ALL_SQL: &'static str = include_str!("./sql/artist/get_all.sql");
impl Artist {
    pub fn get_all(db: &mut DbConnection) -> Result<Vec<Artist>, rusqlite::Error> {
        let c = db.get_connection();
        let mut stmt = c.prepare(GET_ALL_SQL)?;
        let artists = stmt.query_map([], |row| {
            let id = value(row, "Id")?;
            let name = value(row, "Name")?;
            let bio = value(row, "Bio")?;
            let active = value(row, "Active")?;
            let createddate = value(row, "CreatedDate")?;
            let lasteditdate = value(row, "LastEditDate")?;
            Ok(Artist { id, name, bio, active, createddate, lasteditdate })
        })?.into_iter().collect();
        return artists;
    }
}
