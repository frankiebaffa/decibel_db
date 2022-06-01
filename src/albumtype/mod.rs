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
    description: Option<String>,
    active: bool,
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
        query("
            insert into albumtypes (
                name
            ) values (
                $1
            )
        ").bind(name)
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
            query("
                update albumtypes
                set
                    description = $1
                where id = $2
            ").bind(desc)
                .bind(self.get_id())
                .execute(db)
                .await?;
            self.description = Some(desc.to_string());
        }
        Ok(())
    }
}
