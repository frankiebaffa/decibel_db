use {
    chrono::{
        DateTime,
        Local,
    },
    crate::db::Database,
    crate::db::AttachedToDatabase,
    worm_derive::Worm,
};
#[derive(Worm)]
#[dbmodel(table(db="Database", schema="DecibelDb", name="Artists", alias="artist"))]
pub struct Artist {
    #[dbcolumn(column(name="Id", primary_key))]
    id: i64,
    #[dbcolumn(column(name="Name", unique_name, insertable))]
    name: String,
    #[dbcolumn(column(name="Bio", insertable))]
    bio: String,
    #[dbcolumn(column(name="Active", active_flag))]
    active: bool,
    #[dbcolumn(column(name="CreatedDate"))]
    createddate: DateTime<Local>,
    #[dbcolumn(column(name="LastEditDate"))]
    lasteditdate: DateTime<Local>,
}
