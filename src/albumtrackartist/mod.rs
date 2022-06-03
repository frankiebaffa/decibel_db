#[cfg(test)]
mod tests;
use {
    chrono::{DateTime, Utc},
    crate::{
        artist::Artist,
        albumtrack::AlbumTrack,
        artisttype::ArtistType,
    },
    sqlx::{
        FromRow,
        query,
        query_as,
        Result,
        SqlitePool,
    },
};
#[derive(FromRow)]
pub struct AlbumTrackArtist {
    id: i64,
    artist_id: i64,
    albumtrack_id: i64,
    artisttype_id: i64,
    active: bool,
    created_date: DateTime<Utc>,
    last_edit_date: DateTime<Utc>,
}
impl AlbumTrackArtist {
    pub fn get_id(&self) -> i64 {
        self.id
    }
    pub fn get_artist_id(&self) -> i64 {
        self.artist_id
    }
    pub fn get_albumtrack_id(&self) -> i64 {
        self.albumtrack_id
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
                albumtrack_id,
                artisttype_id,
                active,
                created_date,
                last_edit_date
            from albumtrackartists
            where id = $1
            limit 1
        ").bind(id)
            .fetch_one(db)
            .await
    }
    pub async fn insert(
        db: &SqlitePool, artist: &Artist, albumtrack: &AlbumTrack,
        artisttype: &ArtistType
    ) -> Result<Self> {
        let now = Utc::now();
        let id = query("
            insert into albumtrackartists (
                artist_id,
                albumtrack_id,
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
            .bind(albumtrack.get_id())
            .bind(artisttype.get_id())
            .bind(now)
            .execute(db)
            .await?
            .last_insert_rowid();
        Self::lookup(db, id).await
    }
}
