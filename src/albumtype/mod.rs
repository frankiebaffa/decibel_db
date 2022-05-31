use sqlx::{
    FromRow,
    Result,
    SqlitePool,
    query,
    query_as,
};
#[derive(FromRow)]
pub struct AlbumType {
    id: i64,
    name: String,
    description: String,
    active: bool,
}
impl AlbumType {
    pub fn get_id(&self) -> i64 {
        self.id
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_description(&self) -> String {
        self.description.clone()
    }
    pub fn get_active(&self) -> bool {
        self.active
    }
    fn name_query() -> String {
        String::from(
            "select
                id,
                name,
                description,
                active
            from albumtypes
            where name = $1
            limit 1
        ")
    }
    async fn get_by_name<'a>(db: &SqlitePool, name: &'a str) -> Result<Self> {
        query_as::<_, Self>(&Self::name_query())
            .bind(name)
            .fetch_one(db)
            .await
    }
    async fn get_by_name_opt<'a>(db: &SqlitePool, name: &'a str) -> Result<Option<Self>> {
        query_as::<_, Self>(&Self::name_query())
            .bind(name)
            .fetch_optional(db)
            .await
    }
    pub async fn insert<'a>(db: &SqlitePool, name: &'a str, description: &'a str) -> Result<Self> {
        query("
            insert into albumtypes (
                name,
                description
            ) values (
                $1,
                $2
            )
        ").bind(name)
            .bind(description)
            .execute(db)
            .await?;
        Self::get_by_name(db, name).await
    }
    pub async fn lookup<'a>(db: &SqlitePool, name: &'a str, desc: Option<&'a str>) -> Result<Self> {
        match desc {
            Some(d) => match Self::get_by_name_opt(db, name).await? {
                Some(a) => Ok(a),
                None => Self::insert(db, name, d).await,
            },
            None => Self::get_by_name(db, name).await,
        }
    }
}
