use {
    chrono::{
        DateTime,
        Utc,
    },
    crate::albumtype::AlbumType,
    worm::derive::Worm,
};
#[derive(Worm)]
#[dbmodel(table(schema="DecibelDb",name="Albums",alias="album"))]
pub struct Album {
    #[dbcolumn(column(name="Id", primary_key))]
    id: i64,
    #[dbcolumn(column(name="AlbumType_Id", foreign_key="AlbumType", insertable))]
    albumtype_id: i64,
    #[dbcolumn(column(name="Name", insertable))]
    name: String,
    #[dbcolumn(column(name="Blurb", insertable))]
    blurb: Option<String>,
    #[dbcolumn(column(name="Active", active_flag))]
    active: bool,
    #[dbcolumn(column(name="CreatedDate", insertable, utc_now))]
    createddate: DateTime<Utc>,
    #[dbcolumn(column(name="LastEditDate", insertable, utc_now))]
    lasteditdate: DateTime<Utc>,
}
