use {
    chrono::{
        DateTime,
        Utc,
    },
    sqlx::{
        FromRow,
        SqlitePool,
        Result,
        query_as,
        query,
    },
};
#[derive(FromRow)]
pub struct ArtistType {
    id: i64,
    name: String,
    descriptor: String,
    description: Option<String>,
    active: bool,
    created_date: DateTime<Utc>,
    last_edit_date: DateTime<Utc>,
}
impl ArtistType {
    pub fn get_id(&self) -> i64 {
        self.id
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_descriptor(&self) -> String {
        self.descriptor.clone()
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
    fn name_query<'a>() -> String {
        String::from(
            "select
                id,
                name,
                descriptor,
                description,
                active,
                created_date,
                last_edit_date
            from artisttypes
            where name = $1
            limit 1"
        )
    }
    pub async fn lookup<'a>(db: &SqlitePool, type_name: &'a str) -> Result<Self> {
        query_as::<_, Self>(&Self::name_query()).bind(type_name)
            .fetch_one(db)
            .await
    }
    pub async fn optional<'a>(db: &SqlitePool, type_name: &'a str) -> Result<Option<Self>> {
        query_as::<_, Self>(&Self::name_query()).bind(type_name)
            .fetch_optional(db)
            .await
    }
    pub async fn insert<'a>(db: &SqlitePool, type_name: &'a str, desc: &'a str) -> Result<Self> {
        let now = Utc::now();
        query("
            insert into artisttypes (
                name,
                descriptor,
                created_date,
                last_edit_date
            ) values (
                $1,
                $2,
                $3,
                $3
            )
        ").bind(type_name)
            .bind(desc)
            .bind(now)
            .execute(db)
            .await?;
        Self::lookup(db, type_name).await
    }
    pub async fn always<'a>(db: &SqlitePool, type_name: &'a str, desc: &'a str) -> Result<Self> {
        match Self::optional(db, type_name).await? {
            Some(a) => Ok(a),
            None => Self::insert(db, type_name, desc).await,
        }
    }
    pub async fn set_description<'a>(&mut self, db: &SqlitePool, desc: &'a str) -> Result<()> {
        if !self.get_description().eq(&Some(desc.to_string())) {
            let now = Utc::now();
            query("
                update artisttypes
                set
                    description = $1,
                    last_edit_date = $2
                where id = $3
            ").bind(desc)
                .bind(now)
                .bind(self.get_id())
                .execute(db)
                .await?;
            self.last_edit_date = now;
            self.description = Some(desc.to_string());
        }
        Ok(())
    }
}
