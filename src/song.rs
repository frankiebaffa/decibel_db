#[cfg(test)]
mod tests;
use {
    chrono::{
        DateTime,
        Utc,
    },
    crate::utils::opt_vec,
    sqlx::{
        FromRow,
        query,
        query_as,
        Result,
        SqlitePool,
    },
};
#[derive(FromRow)]
pub struct Song {
    id: i64,
    name: String,
    blurb: Option<String>,
    created_date: DateTime<Utc>,
    last_edit_date: DateTime<Utc>,
}
impl Song {
    pub fn get_id(&self) -> i64 {
        self.id
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_blurb(&self) -> Option<String> {
        self.blurb.clone()
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
                name,
                blurb,
                created_date,
                last_edit_date
            from songs
            where id = $1
            limit 1
        ").bind(id)
            .fetch_optional(db)
            .await
    }
    pub async fn load_from_ids(
        db: &SqlitePool,
        ids: Vec<i64>,
    ) -> Result<Option<Vec<Self>>> {
        let mut trn = db.begin()
            .await?;
        query("
            create temp table temp.tmp_song_ids (
                song_id integer not null primary key
            );
        ").execute(&mut trn)
            .await?;
        let mut ids_iter = ids.iter();
        while let Some(id) = ids_iter.next() {
            query("
                insert into temp.tmp_song_ids (
                    song_id
                ) values (
                    $1
                );
            ").bind(&id)
                .execute(&mut trn)
                .await?;
        }
        let songs = query_as::<_, Self>("
            select
                id,
                name,
                blurb,
                created_date,
                last_edit_date
            from songs as song
            join temp.tmp_song_ids as tmp
            on song.id = tmp.song_id;
        ").fetch_all(&mut trn)
            .await?;
        trn.commit().await?;
        Ok(opt_vec(songs))
    }
    pub async fn insert<'a>(
        db: &SqlitePool, name: &'a str
    ) -> Result<i64> {
        let now = Utc::now();
        let id = query("
            insert into songs (
                name,
                created_date,
                last_edit_date
            ) values (
                $1,
                $2,
                $2
            )
        ").bind(name)
            .bind(now)
            .execute(db)
            .await?
            .last_insert_rowid();
        Ok(id)
    }
    pub async fn update_blurb<'a>(
        &mut self, db: &SqlitePool, blurb: &'a str
    ) -> Result<()> {
        if !self.get_blurb().eq(&Some(blurb.to_string())) {
            let now = Utc::now();
            query("
                update songs
                set
                    blurb = $1,
                    last_edit_date = $2
                where id = $3
            ").bind(blurb)
                .bind(now)
                .bind(self.get_id())
                .execute(db)
                .await?;
            self.blurb = Some(blurb.to_string());
            self.last_edit_date = now;
        }
        Ok(())
    }
}
