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
    worm::traits::{
        activeflag::ActiveFlag,
        dbmodel::DbModel,
        foreignkey::ForeignKey,
        helpers::ColumnValue,
        primarykey::PrimaryKey,
    },
    rusqlite::{
        Error,
        Row,
    },
};
pub struct AlbumArtist {
    id: i64,
    artist_id: i64,
    album_id: i64,
    artisttype_id: i64,
    active: bool,
    createddate: DateTime<Local>,
    lasteditdate: DateTime<Local>,
}
impl DbModel for AlbumArtist {
    const TABLE: &'static str = "DecibelDb.AlbumArtist";
    const ALIAS: &'static str = "albumartist";
    fn from_row(row: &Row) -> Result<Self, Error> {
        let id = row.value("Id")?;
        let artist_id = row.value("Artist_Id")?;
        let album_id = row.value("Album_Id")?;
        let artisttype_id = row.value("ArtistType_Id")?;
        let active = row.value("Active")?;
        let createddate = row.value("CreatedDate")?;
        let lasteditdate = row.value("LastEditDate")?;
        Ok(AlbumArtist { id, artist_id, album_id, artisttype_id, active, createddate, lasteditdate })
    }
}
impl PrimaryKey for AlbumArtist {
    const PRIMARY_KEY: &'static str = "Id";
    fn get_id(&self) -> i64 {
        return self.id;
    }
}
impl ActiveFlag for AlbumArtist {
    const ACTIVE: &'static str = "Active";
    fn get_active(&self) -> bool {
        return self.active;
    }
}
impl ForeignKey<Artist> for AlbumArtist {
    const FOREIGN_KEY: &'static str = "Artist_Id";
    const FOREIGN_KEY_PARAM: &'static str = ":artist_id";
    fn get_fk_value(&self) -> i64 {
        return self.artist_id;
    }
}
impl ForeignKey<Album> for AlbumArtist {
    const FOREIGN_KEY: &'static str = "Album_Id";
    const FOREIGN_KEY_PARAM: &'static str = ":album_id";
    fn get_fk_value(&self) -> i64 {
        return self.album_id;
    }
}
impl ForeignKey<ArtistType> for AlbumArtist {
    const FOREIGN_KEY: &'static str = "ArtistType_Id";
    const FOREIGN_KEY_PARAM: &'static str = ":artisttype_id";
    fn get_fk_value(&self) -> i64 {
        return self.artisttype_id;
    }
}
