use {
    chrono::{
        DateTime,
        Local,
    },
    crate::albumtype::AlbumType,
    worm::traits::{
        activeflag::ActiveFlag,
        dbmodel::DbModel,
        foreignkey::ForeignKey,
        helpers::ColumnValue,
        primarykey::PrimaryKey,
    },
    rusqlite::{
        Error,
        Row,
    },
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
    const TABLE: &'static str = "DecibelDb.Album";
    const ALIAS: &'static str = "album";
    fn from_row(row: &Row) -> Result<Self, Error> {
        let id = row.value("Id")?;
        let albumtype_id = row.value("AlbumType_Id")?;
        let name = row.value("Name")?;
        let blurb = row.value("Blurb")?;
        let active = row.value("Active")?;
        let createddate = row.value("CreatedDate")?;
        let lasteditdate = row.value("LastEditDate")?;
        Ok(Self { id, albumtype_id, name, blurb, active, createddate, lasteditdate })
    }
}
impl PrimaryKey for Album {
    const PRIMARY_KEY: &'static str = "Id";
    fn get_id(&self) -> i64 {
        return self.id;
    }
}
impl ActiveFlag for Album {
    const ACTIVE: &'static str = "Active";
    fn get_active(&self) -> bool {
        return self.active;
    }
}
impl ForeignKey<AlbumType> for Album {
    const FOREIGN_KEY: &'static str = "AlbumType_Id";
    const FOREIGN_KEY_PARAM: &'static str = ":albumtype_id";
    fn get_fk_value(&self) -> i64 {
        return self.albumtype_id;
    }
}
