#[cfg(test)]
mod tests;
use {
    chrono::{DateTime, Utc},
    crate::{
        artist::Artist,
        albumtrack::AlbumTrack,
        artisttype::ArtistType,
        utils::opt_vec,
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
    pub fn get_created_date(&self) -> DateTime<Utc> {
        self.created_date
    }
    pub fn get_last_edit_date(&self) -> DateTime<Utc> {
        self.last_edit_date
    }
    pub async fn lookup_by_id(db: &SqlitePool, id: i64) -> Result<Option<Self>> {
        query_as::<_, Self>("
            select
                id,
                artist_id,
                albumtrack_id,
                artisttype_id,
                created_date,
                last_edit_date
            from albumtrackartists
            where id = $1
            limit 1
        ").bind(id)
            .fetch_optional(db)
            .await
    }
    pub async fn insert(
        db: &SqlitePool, artist: &Artist, albumtrack: &AlbumTrack,
        artisttype: &ArtistType
    ) -> Result<i64> {
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
        Ok(id)
    }
    pub async fn load_from_albumtrack_id(
        db: &SqlitePool,
        albumtrackartist_id: i64
    ) -> Result<Option<Vec<Self>>> {
        let albumtrackartists = query_as::<_, Self>("
            select
                id,
                artist_id,
                albumtrack_id,
                artisttype_id,
                created_date,
                last_edit_date
            from albumtrackartist
            where albumtrack_id = $1
        ").bind(albumtrackartist_id)
            .fetch_all(db)
            .await?;
        Ok(opt_vec(albumtrackartists))
    }
    pub async fn load_from_albumtrack_ids(
        db: &SqlitePool,
        albumtrack_ids: Vec<i64>
    ) -> Result<Option<Vec<Self>>> {
        match albumtrack_ids.len() {
            1 => {
                return Self::load_from_albumtrack_id(
                    db,
                    albumtrack_ids.into_iter()
                        .nth(0)
                        .unwrap()
                ).await;
            },
            _ => {},
        }
        let mut trn = db.begin()
            .await?;
        query("
            create temp table temp.tmp_albumtrack_ids (
                albumtrack_id integer not null primary key
            );
        ").execute(&mut trn)
            .await?;
        let mut ids_iter = albumtrack_ids.iter();
        while let Some(id) = ids_iter.next() {
            query("
                insert into temp.tmp_albumtrack_ids (
                    albumtrack_id
                ) values (
                    $1
                );
            ").bind(&id)
                .execute(&mut trn)
                .await?;
        }
        let albums = query_as::<_, Self>("
            select
                id,
                artist_id,
                albumtrack_id,
                artisttype_id,
                created_date,
                last_edit_date
            from albumtrackartist as ata
            join temp.tmp_album_ids as tmp
            on ata.albumtrack_id = tmp.albumtrack_id;
        ").fetch_all(&mut trn)
            .await?;
        trn.commit().await?;
        Ok(opt_vec(albums))
    }
}
