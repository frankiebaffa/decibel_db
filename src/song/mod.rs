use {
    chrono::{
        DateTime,
        Local,
    },
    worm::derive::Worm,
};
#[derive(Worm)]
#[dbmodel(table(schema="DecibelDb",name="Songs",alias="song"))]
pub struct Song {
    #[dbcolumn(column(name="Id", primary_key))]
    id: i64,
    #[dbcolumn(column(name="Name"))]
    name: String,
    #[dbcolumn(column(name="Blurb"))]
    blurb: String,
    #[dbcolumn(column(name="Active", active_flag))]
    active: bool,
    #[dbcolumn(column(name="CreatedDate"))]
    createddate: DateTime<Local>,
    #[dbcolumn(column(name="LastEditDate"))]
    lasteditdate: DateTime<Local>,
}
