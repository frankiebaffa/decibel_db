use chrono::{
    DateTime,
    Local
};
use crate::{
    db::traits::{
        activeflag::ActiveFlag,
        dbmodel::DbModel,
        primarykey::PrimaryKey,
        uniquename::UniqueName,
    },
    sql_utils::value,
};
use rusqlite::{
    Error,
    Row,
};
pub struct AlbumType {
    id: i64,
    name: String,
    description: String,
    active: bool,
    createddate: DateTime<Local>,
    lasteditdate: DateTime<Local>,
}
impl DbModel for AlbumType {
    fn from_row(row: &Row) -> Result<Self, Error> {
        let id = value(row, "Id")?;
        let name = value(row, "Name")?;
        let description = value(row, "Description")?;
        let active = value(row, "Active")?;
        let createddate = value(row, "CreatedDate")?;
        let lasteditdate = value(row, "LastEditDate")?;
        return Ok(AlbumType { id, name, description, active, createddate, lasteditdate });
    }
}
impl PrimaryKey for AlbumType {
    fn get_by_id_sql() -> &'static str {
        return include_str!("./sql/get_by_id.sql");
    }
    fn get_id(&self) -> i64 {
        return self.id;
    }
}
impl UniqueName for AlbumType {
    fn get_by_name_sql() -> &'static str {
        return include_str!("./sql/get_by_name.sql");
    }
}
impl ActiveFlag for AlbumType {
    fn get_all_active_sql() -> &'static str {
        return include_str!("./sql/get_all_active.sql");
    }
}
