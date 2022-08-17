#[cfg(test)]
mod tests;
use {
    chrono::{
        DateTime,
        Utc,
    },
    crate::{
        albumartist::AlbumArtist,
        albumtype::AlbumType,
        file::File,
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
pub struct Album {
    id: i64,
    albumtype_id: i64,
    name: String,
    blurb: Option<String>,
    cover_id: Option<i64>,
    release_date: Option<DateTime<Utc>>,
    created_date: DateTime<Utc>,
    last_edit_date: DateTime<Utc>,
}
impl Album {
    pub fn get_id(&self) -> i64 {
        self.id
    }
    pub fn get_albumtype_id(&self) -> i64 {
        self.albumtype_id
    }
    pub fn get_cover_id(&self) -> Option<i64> {
        self.cover_id
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_blurb(&self) -> Option<String> {
        self.blurb.clone()
    }
    pub fn get_release_date(&self) -> Option<DateTime<Utc>> {
        self.release_date
    }
    pub fn get_created_date(&self) -> DateTime<Utc> {
        self.created_date
    }
    pub fn get_last_edit_date(&self) -> DateTime<Utc> {
        self.last_edit_date
    }
    pub async fn lookup_by_id(db: &SqlitePool, id: i64) -> Result<Self> {
        query_as::<_, Self>("
            select
                id,
                albumtype_id,
                cover_id,
                name,
                blurb,
                release_date,
                created_date,
                last_edit_date
            from albums
            where id = $1;
        ").bind(id)
            .fetch_one(db)
            .await
    }
    pub async fn load_from_ids(
        db: &SqlitePool,
        ids: Vec<i64>,
    ) -> Result<Vec<Self>> {
        match ids.len() {
            1 => {
                let album = Self::lookup_by_id(
                    db,
                    ids.into_iter().nth(0).unwrap()
                ).await?;
                return Ok(vec![album]);
            }
            _ => {},
        }
        let mut trn = db.begin()
            .await?;
        query("
            create temp table temp.tmp_album_ids (
                album_id integer not null primary key
            );
        ").execute(&mut trn)
            .await?;
        let mut ids_iter = ids.iter();
        while let Some(id) = ids_iter.next() {
            query("
                insert into temp.tmp_album_ids (
                    album_id
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
                albumtype_id,
                cover_id,
                name,
                blurb,
                release_date,
                created_date,
                last_edit_date
            from albums as album
            join temp.tmp_album_ids as tmp
            on album.id = tmp.album_id;
        ").fetch_all(&mut trn)
            .await?;
        trn.commit().await?;
        Ok(albums)
    }
    pub async fn load_from_albumartists(
        db: &SqlitePool,
        albumartist: Vec<AlbumArtist>,
    ) -> Result<Vec<Self>> {
        Self::load_from_ids(
            db,
            albumartist.iter()
                .map(|a| a.get_album_id())
                .collect::<Vec<i64>>()
        ).await
    }
    pub async fn lookup_by_id_and_name(
        db: &SqlitePool,
        id: i64,
        name_ref: impl AsRef<str>,
    ) -> Result<Self> {
        let name = name_ref.as_ref();
        query_as::<_, Self>("
            select
                id,
                albumtype_id,
                cover_id,
                name,
                blurb,
                release_date,
                created_date,
                last_edit_date
            from albums as album
            where album.id = $1
            and album.name = $2
        ").bind(id)
            .bind(name)
            .fetch_one(db)
            .await
    }
    pub async fn lookup_by_albumartist_and_name(
        db: &SqlitePool,
        albumartist: AlbumArtist,
        name_ref: impl AsRef<str>,
    ) -> Result<Self> {
        Self::lookup_by_id_and_name(db, albumartist.get_album_id(), name_ref)
            .await
    }
    pub async fn load_from_ids_and_name(
        db: &SqlitePool,
        ids: Vec<i64>,
        name_ref: impl AsRef<str>,
    ) -> Result<Vec<Self>> {
        match ids.len() {
            1 => {
                let album = Self::lookup_by_id_and_name(
                    db,
                    ids.into_iter().nth(0).unwrap(),
                    name_ref,
                ).await?;
                return Ok(vec![album]);
            },
            _ => {},
        }
        let name = name_ref.as_ref();
        let mut trn = db.begin()
            .await?;
        query("
            create temp table temp.tmp_album_ids (
                album_id integer not null primary key
            );
        ").execute(&mut trn)
            .await?;
        let mut ids_iter = ids.iter();
        while let Some(id) = ids_iter.next() {
            query("
                insert into temp.tmp_album_ids (
                    album_id
                ) values (
                    $1
                );
            ").bind(id)
                .execute(&mut trn)
                .await?;
        }
        let albums = query_as::<_, Self>("
            select
                id,
                albumtype_id,
                cover_id,
                name,
                blurb,
                release_date,
                created_date,
                last_edit_date
            from albums as album
            join temp.tmp_album_ids as tmp
            on album.id = tmp.album_id
            and album.name = $1
        ").bind(name)
            .fetch_all(&mut trn)
            .await?;
        trn.commit().await?;
        Ok(albums)
    }
    pub async fn load_from_albumartists_and_name(
        db: &SqlitePool,
        albumartists: Vec<AlbumArtist>,
        name_ref: impl AsRef<str>,
    ) -> Result<Vec<Self>> {
        Self::load_from_ids_and_name(
            db,
            albumartists.into_iter()
                .map(|a| a.get_album_id())
                .collect::<Vec<i64>>(),
            name_ref
        ).await
    }
    pub async fn insert<'a>(
        db: &SqlitePool, album_type: &AlbumType, name: &'a str
    ) -> Result<i64> {
        let now = Utc::now();
        let id = query("
            insert into albums (
                albumtype_id,
                name,
                created_date,
                last_edit_date
            ) values (
                $1,
                $2,
                $3,
                $3
            )
        ").bind(album_type.get_id())
            .bind(name)
            .bind(now)
            .execute(db)
            .await?
            .last_insert_rowid();
        Ok(id)
    }
    pub async fn set_cover(
        &mut self, db: &SqlitePool, cover: &File
    ) -> Result<()> {
        if !self.get_cover_id().eq(&Some(cover.get_id())) {
            let now = Utc::now();
            query("
                update albums
                set
                    cover_id = $1,
                    last_edit_date = $2
                where id = $3
            ").bind(cover.get_id())
                .bind(now)
                .bind(self.get_id())
                .execute(db)
                .await?;
            self.cover_id = Some(cover.get_id());
            self.last_edit_date = now;
        }
        Ok(())
    }
}
