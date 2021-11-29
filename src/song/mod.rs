use {
    chrono::{
        DateTime,
        Utc,
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
    #[dbcolumn(column(name="CreatedDate", insertable, utc_now))]
    createddate: DateTime<Utc>,
    #[dbcolumn(column(name="LastEditDate", insertable, utc_now))]
    lasteditdate: DateTime<Utc>,
}
