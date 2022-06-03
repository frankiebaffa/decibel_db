#[cfg(test)]
mod tests;
use {
    chrono::{DateTime, Utc},
    crate::{
        album::Album,
        file::File,
        song::Song,
    },
    sqlx::{
        FromRow,
        Result,
        query,
        query_as,
        SqlitePool,
    },
};
#[derive(FromRow)]
pub struct AlbumTrack {
    id: i64,
    album_id: i64,
    song_id: i64,
    file_id: Option<i64>,
    track_number: i8,
    version: Option<String>,
    active: bool,
    created_date: DateTime<Utc>,
    last_edit_date: DateTime<Utc>,
}
impl AlbumTrack {
    pub fn get_id(&self) -> i64 {
        self.id
    }
    pub fn get_album_id(&self) -> i64 {
        self.album_id
    }
    pub fn get_song_id(&self) -> i64 {
        self.song_id
    }
    pub fn get_file_id(&self) -> Option<i64> {
        self.file_id
    }
    pub fn get_track_number(&self) -> i8 {
        self.track_number
    }
    pub fn get_version(&self) -> Option<String> {
        self.version.clone()
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
    pub async fn from_album(db: &SqlitePool, album: &Album) -> Result<Vec<Self>> {
        query_as::<_, Self>("
            select
                id,
                album_id,
                song_id,
                file_id,
                track_number,
                version,
                active,
                created_date,
                last_edit_date
            from albumtracks
            where album_id = $1
        ").bind(album.get_id())
            .fetch_all(db)
            .await
    }
    pub async fn lookup(db: &SqlitePool, id: i64) -> Result<Self> {
        query_as::<_, Self>("
            select
                id,
                album_id,
                song_id,
                file_id,
                track_number,
                version,
                active,
                created_date,
                last_edit_date
            from albumtracks
            where id = $1
            limit 1
        ").bind(id)
            .fetch_one(db)
            .await
    }
    pub async fn insert(
        db: &SqlitePool, album: &Album, song: &Song, track_number: i8
    ) -> Result<Self> {
        let now = Utc::now();
        let id = query("
            insert into albumtracks (
                album_id,
                song_id,
                track_number,
                created_date,
                last_edit_date
            ) values (
                $1,
                $2,
                $3,
                $4,
                $4
            )
        ").bind(album.get_id())
            .bind(song.get_id())
            .bind(track_number)
            .bind(now)
            .execute(db)
            .await?
            .last_insert_rowid();
        Self::lookup(db, id).await
    }
    pub async fn set_version<'a>(
        &mut self, db: &SqlitePool, version: &'a str
    ) -> Result<()> {
        if !self.get_version().eq(&Some(version.to_string())) {
            let now = Utc::now();
            query("
                update albumtracks
                set
                    version = $1,
                    last_edit_date = $2
                where id = $3
            ").bind(version)
                .bind(now)
                .bind(self.get_id())
                .execute(db)
                .await?;
            self.version = Some(version.to_string());
            self.last_edit_date = now;
        }
        Ok(())
    }
    pub async fn set_file(
        &mut self, db: &SqlitePool, file: &File
    ) -> Result<()> {
        if !self.get_file_id().eq(&Some(file.get_id())) {
            let now = Utc::now();
            query("
                update albumtracks
                set
                    file_id = $1,
                    last_edit_date = $2
                where id = $3
            ").bind(file.get_id())
                .bind(now)
                .bind(self.get_id())
                .execute(db)
                .await?;
            self.file_id = Some(file.get_id());
            self.last_edit_date = now;
        }
        Ok(())
    }
}
