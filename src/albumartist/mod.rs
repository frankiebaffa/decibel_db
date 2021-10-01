use {
    chrono::{
        DateTime,
        Local,
    },
    crate::{
        artist::Artist,
        album::Album,
        artisttype::ArtistType,
        db::{
            Database,
            AttachedToDatabase,
        },
    },
    worm_derive::Worm,
};
#[derive(Worm)]
#[dbmodel(table(db="Database",schema="DecibelDb",name="AlbumArtists",alias="albumartist"))]
pub struct AlbumArtist {
    #[dbcolumn(column(name="Id", primary_key))]
    id: i64,
    #[dbcolumn(column(name="Artist_Id", foreign_key="Artist"))]
    artist_id: i64,
    #[dbcolumn(column(name="Album_Id", foreign_key="Album"))]
    album_id: i64,
    #[dbcolumn(column(name="ArtistType_Id", foreign_key="ArtistType"))]
    artisttype_id: i64,
    #[dbcolumn(column(name="Active", active_flag))]
    active: bool,
    #[dbcolumn(column(name="CreatedDate"))]
    createddate: DateTime<Local>,
    #[dbcolumn(column(name="LastEditDate"))]
    lasteditdate: DateTime<Local>,
}

