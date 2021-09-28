use chrono::{DateTime, Local};
use crate::{
    db::traits::{
        dbmodel::DbModel,
        primarykey::PrimaryKey,
        uniquename::UniqueName,
    },
    sql_utils::value
};
pub struct ArtistType {
    id: i64,
    name: String,
    descriptor: String,
    description: String,
    active: bool,
    createddate: DateTime<Local>,
    lasteditdate: DateTime<Local>,
}
impl DbModel for ArtistType {
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
impl PrimaryKey for ArtistType {
    fn get_by_id_sql() -> &'static str {
        return include_str!("./sql/get_by_id.sql");
    }
    fn get_id(&self) -> i64 {
        return self.id;
    }
}
impl UniqueName for ArtistType {
    fn get_by_name_sql() -> &'static str {
        return include_str!("./sql/get_by_name.sql");
    }
}
impl ArtistType {
    pub const PERFORMER: &'static str = "Performer";
    pub const WRITER: &'static str = "Writer";
    pub const COMPOSER: &'static str = "Composer";
    pub const PRODUCER: &'static str = "Producer";
    pub const FEATURE: &'static str = "Feature";
}
