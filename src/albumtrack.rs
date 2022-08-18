#[cfg(test)]
mod tests;
use {
    chrono::{DateTime, Utc},
    crate::{
        album::Album,
        file::File,
        song::Song,
        utils::opt_vec,
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
    created_date: DateTime<Utc>,
    last_edit_date: DateTime<Utc>,
}
impl AlbumTrack {
    pub async fn delete(self, db: &SqlitePool) -> Result<()> {
        query(concat!(
            "delete ",
            "from albumtracks ",
            "where id = $1;"
        )).bind(self.id)
            .execute(db)
            .await?;
        Ok(())
    }
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
    pub fn get_created_date(&self) -> DateTime<Utc> {
        self.created_date
    }
    pub fn get_last_edit_date(&self) -> DateTime<Utc> {
        self.last_edit_date
    }
    pub async fn update_version<'a>(
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
    pub async fn update_file_id(
        &mut self, db: &SqlitePool, file_id: i64
    ) -> Result<()> {
        if !self.get_file_id().eq(&Some(file_id)) {
            let now = Utc::now();
            query("
                update albumtracks
                set
                    file_id = $1,
                    last_edit_date = $2
                where id = $3
            ").bind(file_id)
                .bind(now)
                .bind(self.get_id())
                .execute(db)
                .await?;
            self.file_id = Some(file_id);
            self.last_edit_date = now;
        }
        Ok(())
    }
    pub async fn update_file(
        &mut self, db: &SqlitePool, file: &File
    ) -> Result<()> {
        self.update_file_id(db, file.get_id()).await
    }
    pub async fn load_from_album(db: &SqlitePool, album: &Album) -> Result<Option<Vec<Self>>> {
        let albums = query_as::<_, Self>("
            select
                id,
                album_id,
                song_id,
                file_id,
                track_number,
                version,
                created_date,
                last_edit_date
            from albumtracks
            where album_id = $1;
        ").bind(album.get_id())
            .fetch_all(db)
            .await?;
        Ok(opt_vec(albums))
    }
    pub async fn lookup_by_id(db: &SqlitePool, id: i64) -> Result<Option<Self>> {
        query_as::<_, Self>("
            select
                id,
                album_id,
                song_id,
                file_id,
                track_number,
                version,
                created_date,
                last_edit_date
            from albumtracks
            where id = $1;
        ").bind(id)
            .fetch_optional(db)
            .await
    }
    pub async fn insert(
        db: &SqlitePool, album: &Album, song: &Song, track_number: i8
    ) -> Result<i64> {
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
            );
        ").bind(album.get_id())
            .bind(song.get_id())
            .bind(track_number)
            .bind(now)
            .execute(db)
            .await?
            .last_insert_rowid();
        Ok(id)
    }
}