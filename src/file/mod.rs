use {
    chrono::{
        DateTime,
        Utc,
    },
    worm::derive::Worm,
};
#[derive(Worm)]
#[dbmodel(table(schema="DecibelDb",name="Files",alias="file"))]
pub struct File {
    #[dbcolumn(column(name="Id", primary_key))]
    id: i64,
    #[dbcolumn(column(name="FileBlob"))]
    fileblob: Vec<u8>,
    #[dbcolumn(column(name="Active", active_flag))]
    active: bool,
    #[dbcolumn(column(name="CreatedDate", insertable, utc_now))]
    createddate: DateTime<Utc>,
    #[dbcolumn(column(name="LastEditDate", insertable, utc_now))]
    lasteditdate: DateTime<Utc>,
}
