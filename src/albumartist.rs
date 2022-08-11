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
    active: bool,
    created_date: DateTime<Utc>,
    last_edit_date: DateTime<Utc>,
}
impl AlbumArtist {
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
    pub fn get_active(&self) -> bool {
        self.active
    }
    pub fn get_created_date(&self) -> DateTime<Utc> {
        self.created_date
    }
    pub fn get_last_edit_date(&self) -> DateTime<Utc> {
        self.last_edit_date
    }
    pub async fn lookup(db: &SqlitePool, id: i64) -> Result<Self> {
        query_as::<_, Self>("
            select
                id,
                artist_id,
                album_id,
                artisttype_id,
                active,
                created_date,
                last_edit_date
            from albumartists
            where id = $1
            limit 1
        ").bind(id)
            .fetch_one(db)
            .await
    }
    pub async fn insert(
        db: &SqlitePool, artist: &Artist, album: &Album, artisttype: &ArtistType
    ) -> Result<Self> {
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
        Self::lookup(db, id).await
    }
}
