use {
    chrono::{
        DateTime,
        Utc,
    },
    sqlx::{
        FromRow,
        SqlitePool,
        Result as SqlxResult,
        query,
        query_as,
    },
};
#[derive(FromRow)]
pub struct Artist {
    id: i64,
    name: String,
    bio: Option<String>,
    active: bool,
    created_date: DateTime<Utc>,
    last_edit_date: DateTime<Utc>,
}
impl Artist {
    pub async fn lookup(db: &SqlitePool, id: i64) -> SqlxResult<Self> {
        query_as::<_, Self>("
            select
                id,
                name,
                bio,
                active,
                created_date,
                last_edit_date
            from artists
            where id = $1
            limit 1
        ").bind(id)
            .fetch_one(db)
            .await
    }
    pub async fn insert<'a>(db: &SqlitePool, name: &'a str) -> SqlxResult<Self> {
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
        Self::lookup(db, id).await
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
    pub fn get_active(&self) -> bool {
        self.active
    }
    pub fn get_created_date(&self) -> DateTime<Utc> {
        self.created_date
    }
    pub fn get_last_edit_date(&self) -> DateTime<Utc> {
        self.last_edit_date
    }
}
