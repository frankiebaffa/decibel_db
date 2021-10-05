use {
    chrono::{
        DateTime,
        Local,
    },
    crate::{
        artist::Artist,
        album::Album,
        artisttype::ArtistType,
    },
    worm_derive::Worm,
};
#[derive(Worm)]
#[dbmodel(table(schema="DecibelDb",name="AlbumArtists",alias="albumartist"))]
pub struct AlbumArtist {
    #[dbcolumn(column(name="Id", primary_key))]
    id: i64,
    #[dbcolumn(column(name="Artist_Id", foreign_key="Artist", insertable))]
    artist_id: i64,
    #[dbcolumn(column(name="Album_Id", foreign_key="Album", insertable))]
    album_id: i64,
    #[dbcolumn(column(name="ArtistType_Id", foreign_key="ArtistType", insertable))]
    artisttype_id: i64,
    #[dbcolumn(column(name="Active", active_flag))]
    active: bool,
    #[dbcolumn(column(name="CreatedDate"))]
    createddate: DateTime<Local>,
    #[dbcolumn(column(name="LastEditDate"))]
    lasteditdate: DateTime<Local>,
}

