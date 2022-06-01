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
pub struct File {
    id: i64,
    file_blob: Vec<u8>,
    mime_type: String,
    active: bool,
    created_date: DateTime<Utc>,
    last_edit_date: DateTime<Utc>,
}
impl File {
    pub fn get_id(&self) -> i64 {
        self.id
    }
    pub fn get_file_blob(&self) -> Vec<u8> {
        self.file_blob.clone()
    }
    pub fn get_mime_type(&self) -> String {
        self.mime_type.clone()
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
                file_blob,
                mime_type,
                active,
                created_date,
                last_edit_date
            from files
            where id = $1
            limit 1
        ").bind(id)
            .fetch_one(db)
            .await
    }
    pub async fn insert<'a>(
        db: &SqlitePool, file_blob: &'a [u8], mime_type: &'a str
    ) -> Result<Self> {
        let now = Utc::now();
        let id = query("
            insert into files (
                file_blob,
                mime_type,
                created_date,
                last_edit_date
            ) values (
                $1,
                $2,
                $3,
                $3
            )
        ").bind(file_blob)
            .bind(mime_type)
            .bind(now)
            .execute(db)
            .await?
            .last_insert_rowid();
        Self::lookup(db, id).await
    }
}
