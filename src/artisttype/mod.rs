use sqlx::{
    FromRow,
    SqlitePool,
    Result as SqlxResult,
    query_as,
    query,
};
#[derive(FromRow)]
pub struct ArtistType {
    id: i64,
    name: String,
    descriptor: String,
    description: Option<String>,
    active: bool,
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
    fn name_query<'a>() -> String {
        String::from(
            "select
                id,
                name,
                descriptor,
                description,
                active
            from artisttypes
            where name = $1
            limit 1"
        )
    }
    pub async fn lookup<'a>(db: &SqlitePool, type_name: &'a str) -> SqlxResult<Self> {
        query_as::<_, Self>(&Self::name_query()).bind(type_name)
            .fetch_one(db)
            .await
    }
    pub async fn optional<'a>(db: &SqlitePool, type_name: &'a str) -> SqlxResult<Option<Self>> {
        query_as::<_, Self>(&Self::name_query()).bind(type_name)
            .fetch_optional(db)
            .await
    }
    pub async fn insert<'a>(db: &SqlitePool, type_name: &'a str, desc: &'a str) -> SqlxResult<Self> {
        query("
            insert into artisttypes (
                name,
                descriptor
            ) values (
                $1,
                $2
            )
        ").bind(type_name)
            .bind(desc)
            .execute(db)
            .await?;
        Self::lookup(db, type_name).await
    }
    pub async fn always<'a>(db: &SqlitePool, type_name: &'a str, desc: &'a str) -> SqlxResult<Self> {
        match Self::optional(db, type_name).await? {
            Some(a) => Ok(a),
            None => Self::insert(db, type_name, desc).await,
        }
    }
}
