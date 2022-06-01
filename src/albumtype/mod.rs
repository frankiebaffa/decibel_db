use {
    chrono::{
        DateTime,
        Utc,
    },
    sqlx::{
        FromRow,
        Result,
        SqlitePool,
        query,
        query_as,
    },
};
#[derive(FromRow)]
pub struct AlbumType {
    id: i64,
    name: String,
    description: Option<String>,
    active: bool,
    created_date: DateTime<Utc>,
    last_edit_date: DateTime<Utc>,
}
impl AlbumType {
    pub fn get_id(&self) -> i64 {
        self.id
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_description(&self) -> Option<String> {
        self.description.clone()
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
    fn name_query() -> String {
        String::from(
            "select
                id,
                name,
                description,
                active,
                created_date,
                last_edit_date
            from albumtypes
            where name = $1
            limit 1
        ")
    }
    pub async fn lookup<'a>(db: &SqlitePool, name: &'a str) -> Result<Self> {
        query_as::<_, Self>(&Self::name_query())
            .bind(name)
            .fetch_one(db)
            .await
    }
    pub async fn optional<'a>(db: &SqlitePool, name: &'a str) -> Result<Option<Self>> {
        query_as::<_, Self>(&Self::name_query())
            .bind(name)
            .fetch_optional(db)
            .await
    }
    pub async fn insert<'a>(db: &SqlitePool, name: &'a str) -> Result<Self> {
        let now = Utc::now();
        query("
            insert into albumtypes (
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
            .await?;
        Self::lookup(db, name).await
    }
    pub async fn always<'a>(db: &SqlitePool, name: &'a str) -> Result<Self> {
        match Self::optional(db, name).await? {
            Some(a) => Ok(a),
            None => Self::insert(db, name).await,
        }
    }
    pub async fn set_description<'a>(
        &mut self, db: &SqlitePool, desc: &'a str
    ) -> Result<()> {
        if !self.get_description().eq(&Some(desc.to_string())) {
            let now = Utc::now();
            query("
                update albumtypes
                set
                    description = $1,
                    last_edit_date = $2
                where id = $2
            ").bind(desc)
                .bind(now)
                .bind(self.get_id())
                .execute(db)
                .await?;
            self.description = Some(desc.to_string());
            self.last_edit_date = now;
        }
        Ok(())
    }
}
