use {
    chrono::{
        DateTime,
        Local,
    },
    worm::traits::{
        activeflag::ActiveFlag,
        dbmodel::DbModel,
        primarykey::PrimaryKey,
        helpers::ColumnValue,
    },
    rusqlite::{
        Error,
        Row,
    },
};
struct File {
    id: i64,
    fileblob: Vec<u8>,
    active: bool,
    createddate: DateTime<Local>,
    lasteditdate: DateTime<Local>,
}
impl DbModel for File {
    const TABLE: &'static str = "DecibelDb.Files";
    const ALIAS: &'static str = "file";
    fn from_row(row: &Row) -> Result<File, Error> {
        let id = row.value("Id")?;
        let fileblob = row.value("FileBlob")?;
        let active = row.value("Active")?;
        let createddate = row.value("CreatedDate")?;
        let lasteditdate = row.value("LastEditDate")?;
        Ok(File { id, fileblob, active, createddate, lasteditdate })
    }
}
impl PrimaryKey for File {
    const PRIMARY_KEY: &'static str = "Id";
    fn get_id(&self) -> i64 {
        return self.id;
    }
}
impl ActiveFlag for File {
    const ACTIVE: &'static str = "Active";
    fn get_active(&self) -> bool {
        return self.active;
    }
}
