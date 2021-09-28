use chrono::{
    DateTime,
    Local,
};
use crate::{
    albumtype::AlbumType,
    db::traits::{
        activeflag::ActiveFlag,
        dbmodel::DbModel,
        primarykey::PrimaryKey,
        foreignkey::ForeignKey,
    },
    sql_utils::value,
};
use rusqlite::{
    Error,
    Row,
};
pub struct Album {
    id: i64,
    albumtype_id: i64,
    name: String,
    blurb: String,
    active: bool,
    createddate: DateTime<Local>,
    lasteditdate: DateTime<Local>,
}
impl DbModel for Album {
    fn from_row(row: &Row) -> Result<Self, Error> {
        let id = value(row, "Id")?;
        let albumtype_id = value(row, "AlbumType_Id")?;
        let name = value(row, "Name")?;
        let blurb = value(row, "Blurb")?;
        let active = value(row, "Active")?;
        let createddate = value(row, "CreatedDate")?;
        let lasteditdate = value(row, "LastEditDate")?;
        Ok(Self { id, albumtype_id, name, blurb, active, createddate, lasteditdate })
    }
}
impl PrimaryKey for Album {
    fn get_by_id_sql() -> &'static str {
        return include_str!("./sql/get_by_id.sql");
    }
    fn get_id(&self) -> i64 {
        return self.id;
    }
}
impl ActiveFlag for Album {
    fn get_all_active_sql() -> &'static str {
        return include_str!("./sql/get_all_active.sql");
    }
}
impl ForeignKey<AlbumType> for Album {
    fn get_fk_name() -> &'static str {
        return ":albumtype_id";
    }
    fn get_fk_value(&self) -> i64 {
        return self.albumtype_id;
    }
    fn get_all_by_fk_sql() -> &'static str {
        return include_str!("./sql/get_all_by_albumtype_id.sql");
    }
    fn get_fk_sql() -> &'static str {
        return include_str!("./sql/get_albumtype.sql");
    }
}
