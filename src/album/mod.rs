use {
    chrono::{
        DateTime,
        Utc,
    },
    crate::{
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
    active: bool,
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
    pub fn get_active(&self) -> bool {
        self.active
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
    pub async fn lookup(db: &SqlitePool, id: i64) -> Result<Self> {
        query_as::<_, Self>("
            select
                id,
                albumtype_id,
                cover_id,
                name,
                blurb,
                active,
                release_date,
                created_date,
                last_edit_date
            from albums
            where id = $1
            limit 1
        ").bind(id)
            .fetch_one(db)
            .await
    }
    pub async fn insert<'a>(
        db: &SqlitePool, album_type: &AlbumType, name: &'a str
    ) -> Result<Self> {
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
        Self::lookup(db, id).await
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
