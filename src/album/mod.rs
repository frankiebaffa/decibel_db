use {
    chrono::{
        DateTime,
        Local,
    },
    crate::{albumtype::AlbumType, db::Database, db::AttachedToDatabase},
    worm_derive::Worm,
};
#[derive(Worm)]
#[dbmodel(table(db="Database", schema="DecibelDb",name="Albums",alias="Album"))]
pub struct Album {
    #[dbcolumn(column(name="Id", primary_key))]
    id: i64,
    #[dbcolumn(column(name="AlbumType_Id", foreign_key="AlbumType"))]
    albumtype_id: i64,
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
