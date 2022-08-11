#[cfg(test)]
mod tests;
use {
    chrono::{
        DateTime,
        Utc,
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
pub struct Song {
    id: i64,
    name: String,
    blurb: Option<String>,
    active: bool,
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
                name,
                blurb,
                active,
                created_date,
                last_edit_date
            from songs
            where id = $1
            limit 1
        ").bind(id)
            .fetch_one(db)
            .await
    }
    pub async fn insert<'a>(
        db: &SqlitePool, name: &'a str
    ) -> Result<Self> {
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
        Self::lookup(db, id).await
    }
    pub async fn set_blurb<'a>(
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
