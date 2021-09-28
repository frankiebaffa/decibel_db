use {
    chrono::{DateTime, Local},
    crate::db::traits::{
        dbmodel::DbModel,
        helpers::ColumnValue,
        primarykey::PrimaryKey,
        uniquename::UniqueName,
    },
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
    const TABLE: &'static str = "DecibelDb.AristType";
    const ALIAS: &'static str = "artisttype";
    fn from_row(row: &rusqlite::Row) -> Result<Self, rusqlite::Error> {
        let id = row.value("Id")?;
        let name = row.value("Name")?;
        let descriptor = row.value("Descriptor")?;
        let description = row.value("Description")?;
        let active = row.value("Active")?;
        let createddate = row.value("CreatedDate")?;
        let lasteditdate = row.value("LastEditDate")?;
        return Ok(ArtistType { id, name, descriptor, description, active, createddate, lasteditdate });
    }
}
impl PrimaryKey for ArtistType {
    const PRIMARY_KEY: &'static str = "Id";
    fn get_id(&self) -> i64 {
        return self.id;
    }
}
impl UniqueName for ArtistType {
    const NAME: &'static str = "Name";
    fn get_name(&self) -> String {
        return self.name.clone();
    }
}
impl ArtistType {
    pub const PERFORMER: &'static str = "Performer";
    pub const WRITER: &'static str = "Writer";
    pub const COMPOSER: &'static str = "Composer";
    pub const PRODUCER: &'static str = "Producer";
    pub const FEATURE: &'static str = "Feature";
}
