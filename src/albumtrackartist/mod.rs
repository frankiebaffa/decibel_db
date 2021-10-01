use {
    chrono::{DateTime, Local},
    crate::{
        artist::Artist,
        albumtrack::AlbumTrack,
        artisttype::ArtistType,
        db::{
            AttachedToDatabase,
            Database,
        },
    },
    worm_derive::Worm,
};
#[derive(Worm)]
#[dbmodel(table(db="Database",schema="DecibelDb", name="AlbumTrackArtists", alias="albumtrackartist"))]
pub struct AlbumTrackArtist {
    #[dbcolumn(column(name="Id", primary_key))]
    id: i64,
    #[dbcolumn(column(name="Artist_Id", foreign_key="Artist"))]
    artist_id: i64,
    #[dbcolumn(column(name="AlbumTrack_Id", foreign_key="AlbumTrack"))]
    albumtrack_id: i64,
    #[dbcolumn(column(name="ArtistType_Id", foreign_key="ArtistType"))]
    artisttype_id: i64,
    #[dbcolumn(column(name="Active", active_flag))]
    active: bool,
    #[dbcolumn(column(name="CreatedDate"))]
    createddate: DateTime<Local>,
    #[dbcolumn(column(name="LastEditDate"))]
    lasteditdate: DateTime<Local>,
}
