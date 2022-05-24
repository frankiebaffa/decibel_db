use sqlx::{
    Error as SqlxError,
    SqlitePool,
    query,
    query_as,
};
pub struct DecibelMigrator;
impl DecibelMigrator {
    async fn tbl_artists(db: &SqlitePool) -> Result<(), SqlxError> {
        let chk = query_as::<_, (i64,)>("
            select count(name)
            from sqlite_master
            where type = 'table'
            and name = 'artists';
        ").fetch_one(db).await? .0 > 0;
        if !chk {
            query("
                create table artists (
                    id integer not null primary key autoincrement,
                    name text not null,
                    bio text null,
                    active integer not null default 1,
                    created_date text not null,
                    last_edit_date text not null
                    );
            ").execute(db).await?;
        }
        Ok(())
    }
    async fn idx_artistsname(db: &SqlitePool) -> Result<(), SqlxError> {
        let chk = query_as::<_, (i64,)>("
            select count(name)
            from sqlite_master
            where type = 'index'
            and name = 'artistsnameunique';
        ").fetch_one(db).await? .0 > 0;
        if !chk {
            query("
                create unique index artistsnameunique
                on artists (name)
                where active = 1;
            ").execute(db).await?;
        }
        Ok(())
    }
    async fn tbl_artisttypes(db: &SqlitePool) -> Result<(), SqlxError> {
        let chk = query_as::<_, (i64,)>("
            select count(name)
            from sqlite_master
            where type = 'table'
            and name = 'artisttypes';
        ").fetch_one(db).await? .0 > 0;
        if !chk {
            query("
                create table artisttypes
                    (
                        id integer not null primary key autoincrement,
                        name text not null,
                        descriptor text not null,
                        description text null,
                        active integer not null default 1
                    );
            ").execute(db).await?;
        }
        Ok(())
    }
    pub async fn migrate(db: &SqlitePool) -> Result<(), SqlxError> {
        Self::tbl_artists(db).await?;
        Self::idx_artistsname(db).await?;
        Self::tbl_artisttypes(db).await?;
        Ok(())
    }
}
