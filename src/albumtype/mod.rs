use {
    chrono::{
        DateTime,
        Local
    },
    worm_derive::Worm,
};
#[derive(Worm)]
#[dbmodel(table(schema="DecibelDb",name="AlbumTypes",alias="albumtype"))]
pub struct AlbumType {
    #[dbcolumn(column(name="Id", primary_key))]
    id: i64,
    #[dbcolumn(column(name="Name", unique_name))]
    name: String,
    #[dbcolumn(column(name="Description"))]
    description: String,
    #[dbcolumn(column(name="Active", active_flag))]
    active: bool,
    #[dbcolumn(column(name="CreatedDate"))]
    createddate: DateTime<Local>,
    #[dbcolumn(column(name="LastEditDate"))]
    lasteditdate: DateTime<Local>,
}
