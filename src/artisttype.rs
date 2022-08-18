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
    created_date: DateTime<Utc>,
    last_edit_date: DateTime<Utc>,
}
impl ArtistType {
    pub async fn delete(self, db: &SqlitePool) -> Result<()> {
        query("
            delete
            from artisttypes
            where id = $1
        ").bind(self.id)
            .execute(db)
            .await?;
        Ok(())
    }
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
    pub fn get_created_date(&self) -> DateTime<Utc> {
        self.created_date
    }
    pub fn get_last_edit_date(&self) -> DateTime<Utc> {
        self.last_edit_date
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
    pub async fn lookup_by_id<'a>(
        db: &SqlitePool,
        id: i64,
    ) -> Result<Option<Self>> {
        query_as::<_, Self>("
            select
                id,
                name,
                descriptor,
                description,
                created_date,
                last_edit_date
            from artisttypes
            where id = $1;
        ").bind(id)
            .fetch_optional(db)
            .await
    }
    pub async fn lookup_by_name<'a>(
        db: &SqlitePool,
        name_ref: impl AsRef<str>,
    ) -> Result<Option<Self>> {
        let name = name_ref.as_ref();
        query_as::<_, Self>("
            select
                id,
                name,
                descriptor,
                description,
                created_date,
                last_edit_date
            from artisttypes
            where name = $1;
        ").bind(name)
            .fetch_optional(db)
            .await
    }
    pub async fn insert<'a>(
        db: &SqlitePool,
        type_name: &'a str,
        descriptor: &'a str
    ) -> Result<i64> {
        let now = Utc::now();
        let id = query("
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
            .bind(descriptor)
            .bind(now)
            .execute(db)
            .await?
            .last_insert_rowid();
        Ok(id)
    }
}
