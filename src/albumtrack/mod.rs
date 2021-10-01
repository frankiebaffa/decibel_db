use {
    chrono::{DateTime, Local},
    crate::{
        album::Album,
        db::{
            AttachedToDatabase,
            Database,
        },
        song::Song,
        file::File,
    },
    worm_derive::Worm,
};
#[derive(Worm)]
#[dbmodel(table(db="Database",schema="DecibelDb", name="AlbumTracks", alias="albumtrack"))]
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
    #[dbcolumn(column(name="CreatedDate"))]
    createddate: DateTime<Local>,
    #[dbcolumn(column(name="LastEditDate"))]
    lasteditdate: DateTime<Local>,
}
