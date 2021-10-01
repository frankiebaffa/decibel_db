use {
    chrono::{
        DateTime,
        Local,
    },
    crate::db::{
        AttachedToDatabase,
        Database,
    },
    worm_derive::Worm,
};
#[derive(Worm)]
#[dbmodel(table(db="Database",schema="DecibelDb",name="Files",alias="file"))]
pub struct File {
    #[dbcolumn(column(name="Id", primary_key))]
    id: i64,
    #[dbcolumn(column(name="FileBlob"))]
    fileblob: Vec<u8>,
    #[dbcolumn(column(name="Active", active_flag))]
    active: bool,
    #[dbcolumn(column(name="CreatedDate"))]
    createddate: DateTime<Local>,
    #[dbcolumn(column(name="LastEditDate"))]
    lasteditdate: DateTime<Local>,
}
