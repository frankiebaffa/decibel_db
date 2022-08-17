#[cfg(test)]
mod tests;
use {
    chrono::{
        DateTime,
        Utc,
    },
    crate::albumtrack::AlbumTrack,
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
    pub fn get_created_date(&self) -> DateTime<Utc> {
        self.created_date
    }
    pub fn get_last_edit_date(&self) -> DateTime<Utc> {
        self.last_edit_date
    }
    pub async fn load_from_albumtracks(
        db: &SqlitePool, albumtracks: &Vec<AlbumTrack>
    ) -> Result<Vec<Self>> {
        // begin trn
        let mut trn = db.begin().await?;
        match query("
            create temp table temp.tmpfileids (
                id integer not null primary key
            )
        ").execute(&mut trn).await {
            Ok(_) => {},
            Err(e) => {
                trn.rollback().await?;
                return Err(e);
            },
        }
        let mut albumtracks_iter = albumtracks.iter();
        while let Some(at) = albumtracks_iter.next() {
            match query("
                insert into temp.tmpfileids (
                    id
                ) values (
                    $1
                )
            ").bind(at.get_file_id())
                .execute(&mut trn)
                .await
            {
                Ok(_) => {},
                Err(e) => {
                    trn.rollback().await?;
                    return Err(e);
                },
            }
        }
        let tracks = match query_as::<_, Self>("
            select
                file.id,
                file.file_blob,
                file.mime_type,
                file.created_date,
                file.last_edit_date
            from files as file
            join temp.tmpfileids as tmp
            on file.id = tmp.id
        ").fetch_all(&mut trn).await {
            Ok(t) => t,
            Err(e) => {
                trn.rollback().await?;
                return Err(e);
            },
        };
        match query("
            drop table temp.tmpfileids
        ").execute(&mut trn).await {
            Ok(_) => {},
            Err(e) => {
                trn.rollback().await?;
                return Err(e);
            },
        }
        trn.commit().await?;
        Ok(tracks)
    }
    pub async fn lookup_by_id(db: &SqlitePool, id: i64) -> Result<Self> {
        query_as::<_, Self>("
            select
                id,
                file_blob,
                mime_type,
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
    ) -> Result<i64> {
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
        Ok(id)
    }
}
