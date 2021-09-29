use {
    chrono::{
        DateTime,
        Local,
    },
    worm::traits::{
        activeflag::ActiveFlag,
        dbmodel::DbModel,
        helpers::ColumnValue,
        primarykey::PrimaryKey,
    },
    rusqlite::{
        Error,
        Row,
    },
};
pub struct Song {
    id: i64,
    name: String,
    blurb: String,
    active: bool,
    createddate: DateTime<Local>,
    lasteditdate: DateTime<Local>,
}
impl DbModel for Song {
    const TABLE: &'static str = "DecibelDb.Songs";
    const ALIAS: &'static str = "song";
    fn from_row(row: &Row) -> Result<Self, Error> {
        let id = row.value("Id")?;
        let name = row.value("Name")?;
        let blurb = row.value("Blurb")?;
        let active = row.value("Active")?;
        let createddate = row.value("CreatedDate")?;
        let lasteditdate = row.value("LastEditDate")?;
        Ok(Song { id, name, blurb, active, createddate, lasteditdate })
    }
}
impl PrimaryKey for Song {
    const PRIMARY_KEY: &'static str = "Id";
    fn get_id(&self) -> i64 {
        return self.id;
    }
}
impl ActiveFlag for Song {
    const ACTIVE: &'static str = "Active";
    fn get_active(&self) -> bool {
        return self.active;
    }
}
