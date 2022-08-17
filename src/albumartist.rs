#[cfg(test)]
mod tests;
use {
    chrono::{
        DateTime,
        Utc,
    },
    crate::{
        artist::Artist,
        artisttype::ArtistType,
        album::Album,
        utils::opt_vec,
    },
    sqlx::{
        FromRow,
        Result,
        SqlitePool,
        query,
        query_as,
    },
};
#[derive(FromRow)]
pub struct AlbumArtist {
    id: i64,
    artist_id: i64,
    album_id: i64,
    artisttype_id: i64,
    created_date: DateTime<Utc>,
    last_edit_date: DateTime<Utc>,
}
impl AlbumArtist {
    pub async fn delete(self, db: &SqlitePool) -> Result<()> {
        query("
            delete
            from albumartists
            where id = $1
        ").bind(self.id)
            .execute(db)
            .await?;
        Ok(())
    }
    pub fn get_id(&self) -> i64 {
        self.id
    }
    pub fn get_artist_id(&self) -> i64 {
        self.artist_id
    }
    pub fn get_album_id(&self) -> i64 {
        self.album_id
    }
    pub fn get_artisttype_id(&self) -> i64 {
        self.artisttype_id
    }
    pub fn get_created_date(&self) -> DateTime<Utc> {
        self.created_date
    }
    pub fn get_last_edit_date(&self) -> DateTime<Utc> {
        self.last_edit_date
    }
    pub async fn lookup_by_id(
        db: &SqlitePool, id: i64
    ) -> Result<Option<Self>> {
        query_as::<_, Self>("
            select
                id,
                artist_id,
                album_id,
                artisttype_id,
                created_date,
                last_edit_date
            from albumartists
            where id = $1;
        ").bind(id)
            .fetch_optional(db)
            .await
    }

    pub async fn load_from_album_id(
        db: &SqlitePool, id: i64
    ) -> Result<Option<Vec<Self>>> {
        let albumartists = query_as::<_, Self>("
            select
                id,
                artist_id,
                album_id,
                artisttype_id,
                created_date,
                last_edit_date
            from albumartists
            where album_id = $1;
        ").bind(id)
            .fetch_all(db)
            .await?;
        Ok(opt_vec(albumartists))
    }
    pub async fn load_from_album(db: &SqlitePool, album: Album) -> Result<Option<Vec<Self>>> {
        Self::load_from_album_id(db, album.get_id()).await
    }
    pub async fn load_from_artist_id(
        db: &SqlitePool,
        id: i64,
    ) -> Result<Option<Vec<Self>>> {
        let albumartists = query_as::<_, Self>("
            select
                id,
                artist_id,
                album_id,
                artisttype_id,
                created_date,
                last_edit_date
            from albumartists
            where artist_id = $1;
        ").bind(id)
            .fetch_all(db)
            .await?;
        Ok(opt_vec(albumartists))
    }
    pub async fn load_from_artist(
        db: &SqlitePool, artist: &Artist
    ) -> Result<Option<Vec<Self>>> {
        Self::load_from_artist_id(db, artist.get_id())
            .await
    }
    pub async fn insert(
        db: &SqlitePool, artist: &Artist, album: &Album, artisttype: &ArtistType
    ) -> Result<i64> {
        let now = Utc::now();
        let id = query("
            insert into albumartists (
                artist_id,
                album_id,
                artisttype_id,
                created_date,
                last_edit_date
            ) values (
                $1,
                $2,
                $3,
                $4,
                $4
            )
        ").bind(artist.get_id())
            .bind(album.get_id())
            .bind(artisttype.get_id())
            .bind(now)
            .execute(db)
            .await?
            .last_insert_rowid();
        Ok(id)
    }
}
