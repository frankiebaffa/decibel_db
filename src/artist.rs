#[cfg(test)]
mod tests;
use {
    chrono::{
        DateTime,
        Utc,
    },
    sqlx::{
        FromRow,
        SqlitePool,
        Result,
        query,
        query_as,
    },
};
#[derive(FromRow)]
pub struct Artist {
    id: i64,
    name: String,
    bio: Option<String>,
    created_date: DateTime<Utc>,
    last_edit_date: DateTime<Utc>,
}
impl Artist {
    pub async fn get_all(db: &SqlitePool) -> Result<Vec<Self>> {
        query_as::<_, Self>("
            select
                id,
                name,
                bio,
                created_date,
                last_edit_date
            from artists;
        ").fetch_all(db)
            .await
    }
    pub async fn lookup_by_id(db: &SqlitePool, id: i64) -> Result<Self> {
        query_as::<_, Self>("
            select
                id,
                name,
                bio,
                active,
                created_date,
                last_edit_date
            from artists
            where id = $1;
        ").bind(id)
            .fetch_one(db)
            .await
    }
    pub async fn insert<'a>(db: &SqlitePool, name: &'a str) -> Result<Self> {
        let now = Utc::now();
        let active = true;
        let id = query("
            insert into artists (
                name,
                active,
                created_date,
                last_edit_date
            ) values (
                $1,
                $2,
                $3,
                $3
            );
        ").bind(name)
            .bind(active)
            .bind(now)
            .execute(db)
            .await?
            .last_insert_rowid();
        Self::lookup_by_id(db, id).await
    }
    pub fn get_id(&self) -> i64 {
        self.id
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_bio(&self) -> Option<String> {
        self.bio.clone()
    }
    pub fn get_created_date(&self) -> DateTime<Utc> {
        self.created_date
    }
    pub fn get_last_edit_date(&self) -> DateTime<Utc> {
        self.last_edit_date
    }
    pub async fn delete(self, db: &SqlitePool) -> Result<()> {
        query("
            delete
            from artists
            where id = $3
        ").bind(0)
            .bind(self.get_id())
            .execute(db)
            .await?;
        Ok(())
    }
}
