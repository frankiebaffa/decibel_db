use {
    chrono::{DateTime, Utc},
    crate::{
        artist::Artist,
        albumtrack::AlbumTrack,
        artisttype::ArtistType,
    },
    worm::derive::Worm,
};
#[derive(Worm)]
#[dbmodel(table(schema="DecibelDb", name="AlbumTrackArtists", alias="albumtrackartist"))]
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
    #[dbcolumn(column(name="CreatedDate", insertable, utc_now))]
    createddate: DateTime<Utc>,
    #[dbcolumn(column(name="LastEditDate", insertable, utc_now))]
    lasteditdate: DateTime<Utc>,
}
