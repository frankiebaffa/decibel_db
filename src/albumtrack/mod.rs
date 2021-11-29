use {
    chrono::{DateTime, Utc},
    crate::{
        album::Album,
        song::Song,
        file::File,
    },
    worm::derive::Worm,
};
#[derive(Worm)]
#[dbmodel(table(schema="DecibelDb", name="AlbumTracks", alias="albumtrack"))]
pub struct AlbumTrack {
    #[dbcolumn(column(name="Id", primary_key))]
    id: i64,
    #[dbcolumn(column(name="Album_Id", foreign_key="Album"))]
    album_id: i64,
    #[dbcolumn(column(name="Song_Id", foreign_key="Song"))]
    song_id: i64,
    #[dbcolumn(column(name="File_Id", foreign_key="File"))]
    file_id: i64,
    #[dbcolumn(column(name="TrackNumber"))]
    tracknumber: i8,
    #[dbcolumn(column(name="Version"))]
    version: String,
    #[dbcolumn(column(name="Active", active_flag))]
    active: bool,
    #[dbcolumn(column(name="CreatedDate", insertable, utc_now))]
    createddate: DateTime<Utc>,
    #[dbcolumn(column(name="LastEditDate", insertable, utc_now))]
    lasteditdate: DateTime<Utc>,
}
