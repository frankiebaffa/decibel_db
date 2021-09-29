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
        primarykey::PrimaryKey,
    },
    worm_derive::Worm,
};
#[derive(Worm)]
#[dbmodel(table(db="DecibelDb",name="AlbumArtists",alias="albumartist"))]
pub struct AlbumArtist {
    #[dbcolumn(column(name="Id"))]
    id: i64,
    #[dbcolumn(column(name="Artist_Id"))]
    artist_id: i64,
    #[dbcolumn(column(name="Album_Id"))]
    album_id: i64,
    #[dbcolumn(column(name="ArtistType_Id"))]
    artisttype_id: i64,
    #[dbcolumn(column(name="Active"))]
    active: bool,
    #[dbcolumn(column(name="CreatedDate"))]
    createddate: DateTime<Local>,
    #[dbcolumn(column(name="LastEditDate"))]
    lasteditdate: DateTime<Local>,
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
