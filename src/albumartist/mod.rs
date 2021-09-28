use chrono::{
    DateTime,
    Local,
};
use crate::{
    artist::Artist,
    album::Album,
    artisttype::ArtistType,
    db::traits::{
        activeflag::ActiveFlag,
        dbmodel::DbModel,
        foreignkey::ForeignKey,
        primarykey::PrimaryKey,
    },
    sql_utils::value,
};
use rusqlite::{
    Error,
    Row,
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
    fn from_row(row: &Row) -> Result<Self, Error> {
        let id = value(row, "Id")?;
        let artist_id = value(row, "Artist_Id")?;
        let album_id = value(row, "Album_Id")?;
        let artisttype_id = value(row, "ArtistType_Id")?;
        let active = value(row, "Active")?;
        let createddate = value(row, "CreatedDate")?;
        let lasteditdate = value(row, "LastEditDate")?;
        Ok(AlbumArtist { id, artist_id, album_id, artisttype_id, active, createddate, lasteditdate })
    }
}
impl PrimaryKey for AlbumArtist {
    fn get_by_id_sql() -> &'static str {
        return include_str!("./sql/get_by_id.sql");
    }
    fn get_id(&self) -> i64 {
        return self.id;
    }
}
impl ActiveFlag for AlbumArtist {
    fn get_all_active_sql() -> &'static str {
        return include_str!("./sql/get_all_active.sql");
    }
}
impl ForeignKey<Artist> for AlbumArtist {
    fn get_fk_name() -> &'static str {
        return ":artist_id";
    }
    fn get_fk_value(&self) -> i64 {
        return self.artist_id;
    }
    fn get_all_by_fk_sql() -> &'static str {
        return include_str!("./sql/get_all_by_artist_id.sql");
    }
    fn get_fk_sql() -> &'static str {
        return include_str!("./sql/get_artist.sql");
    }
}
impl ForeignKey<Album> for AlbumArtist {
    fn get_fk_name() -> &'static str {
        return ":album_id";
    }
    fn get_fk_value(&self) -> i64 {
        return self.album_id;
    }
    fn get_all_by_fk_sql() -> &'static str {
        return include_str!("./sql/get_all_by_album_id.sql");
    }
    fn get_fk_sql() -> &'static str {
        return include_str!("./sql/get_album.sql");
    }
}
impl ForeignKey<ArtistType> for AlbumArtist {
    fn get_fk_name() -> &'static str {
        return ":album_id";
    }
    fn get_fk_value(&self) -> i64 {
        return self.album_id;
    }
    fn get_all_by_fk_sql() -> &'static str {
        return include_str!("./sql/get_all_by_artisttype_id.sql");
    }
    fn get_fk_sql() -> &'static str {
        return include_str!("./sql/get_artisttype.sql");
    }
}
