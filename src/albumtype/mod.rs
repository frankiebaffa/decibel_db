use {
    chrono::{
        DateTime,
        Local
    },
    crate::db::traits::{
        activeflag::ActiveFlag,
        dbmodel::DbModel,
        helpers::ColumnValue,
        primarykey::PrimaryKey,
        uniquename::UniqueName,
    },
    rusqlite::{
        Error,
        Row,
    },
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
    const TABLE: &'static str = "DecibelDb.AlbumType";
    const ALIAS: &'static str = "albumtype";
    fn from_row(row: &Row) -> Result<Self, Error> {
        let id = row.value("Id")?;
        let name = row.value("Name")?;
        let description = row.value("Description")?;
        let active = row.value("Active")?;
        let createddate = row.value("CreatedDate")?;
        let lasteditdate = row.value("LastEditDate")?;
        return Ok(AlbumType { id, name, description, active, createddate, lasteditdate });
    }
}
impl PrimaryKey for AlbumType {
    const PRIMARY_KEY: &'static str = "Id";
    fn get_id(&self) -> i64 {
        return self.id;
    }
}
impl UniqueName for AlbumType {
    const NAME: &'static str = "Name";
    fn get_name(&self) -> String {
        return self.name.clone();
    }
}
impl ActiveFlag for AlbumType {
    const ACTIVE: &'static str = "Active";
    fn get_active(&self) -> bool {
        return self.active;
    }
}
