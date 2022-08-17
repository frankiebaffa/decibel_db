#[cfg(test)]
mod tests;
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
    created_date: DateTime<Utc>,
    last_edit_date: DateTime<Utc>,
}
impl AlbumType {
    pub async fn delete(self, db: &SqlitePool) -> Result<()> {
        query("
            delete
            from albumtypes
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
    pub fn get_description(&self) -> Option<String> {
        self.description.clone()
    }
    pub fn get_created_date(&self) -> DateTime<Utc> {
        self.created_date
    }
    pub fn get_last_edit_date(&self) -> DateTime<Utc> {
        self.last_edit_date
    }
    pub async fn lookup_by_id<'a>(
        db: &SqlitePool, id: i64
    ) -> Result<Option<Self>> {
        query_as::<_, Self>("
            select
                id,
                name,
                description,
                created_date,
                last_edit_date
            from albumtypes
            where id = $1;
        ").bind(id)
            .fetch_optional(db)
            .await
    }
    pub async fn lookup_by_name<'a>(
        db: &SqlitePool, name: &'a str
    ) -> Result<Option<Self>> {
        query_as::<_, Self>("
            select
                id,
                name,
                description,
                created_date,
                last_edit_date
            from albumtypes
            where name = $1;
        ").bind(name)
            .fetch_optional(db)
            .await
    }
    pub async fn insert<'a>(db: &SqlitePool, name: &'a str) -> Result<i64> {
        let now = Utc::now();
        let id = query("
            insert into albumtypes (
                name,
                created_date,
                last_edit_date
            ) values (
                $1,
                $2,
                $2
            );
        ").bind(name)
            .bind(now)
            .execute(db)
            .await?
            .last_insert_rowid();
        Ok(id)
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
