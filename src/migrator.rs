use sqlx::{
    Error as SqlxError,
    SqlitePool,
    query,
};
pub struct DecibelMigrator;
impl DecibelMigrator {
    async fn tbl_artists(db: &SqlitePool) -> Result<(), SqlxError> {
        query("
            create table if not exists artists (
                id integer not null primary key autoincrement,
                name text not null,
                bio text null,
                created_date text not null,
                last_edit_date text not null
            );
        ").execute(db).await?;
        Ok(())
    }
    async fn idx_artistsname(db: &SqlitePool) -> Result<(), SqlxError> {
        query("
            create unique index if not exists artistsnameunique
            on artists (name);
        ").execute(db).await?;
        Ok(())
    }
    async fn tbl_artisttypes(db: &SqlitePool) -> Result<(), SqlxError> {
        query("
            create table if not exists artisttypes
                (
                    id integer not null primary key autoincrement,
                    name text not null,
                    descriptor text not null,
                    description text null,
                    created_date text not null,
                    last_edit_date text not null
                );
        ").execute(db).await?;
        Ok(())
    }
    async fn idx_artisttypesname(db: &SqlitePool) -> Result<(), SqlxError> {
        query("
            create unique index if not exists artisttypesnameunique
            on artisttypes (name);
        ").execute(db).await?;
        Ok(())
    }
    async fn tbl_albumtypes(db: &SqlitePool) -> Result<(), SqlxError> {
        query("
            create table if not exists albumtypes (
                id integer not null primary key autoincrement,
                name text not null,
                description text null,
                created_date text not null,
                last_edit_date text not null
            );
        ").execute(db).await?;
        Ok(())
    }
    async fn idx_albumtypesname(db: &SqlitePool) -> Result<(), SqlxError> {
        query("
            create unique index if not exists albumtypesnameunique
            on albumtypes (name);
        ").execute(db).await?;
        Ok(())
    }
    async fn tbl_albums(db: &SqlitePool) -> Result<(), SqlxError> {
        query("
            create table if not exists albums (
                id integer not null primary key autoincrement,
                albumtype_id integer not null,
                name text not null,
                blurb text null,
                cover_id integer null,
                release_date text null,
                created_date text not null,
                last_edit_date text not null,
                foreign key (albumtype_id) references albumtypes(id),
                foreign key (cover_id) references files (id)
            );
        ").execute(db).await?;
        Ok(())
    }
    async fn tbl_albumartists(db: &SqlitePool) -> Result<(), SqlxError> {
        query("
            create table if not exists albumartists (
                id integer not null primary key autoincrement,
                artist_id integer not null,
                album_id integer not null,
                artisttype_id integer not null,
                created_date text not null,
                last_edit_date text not null,
                foreign key (artist_id) references artists (id),
                foreign key (album_id) references albums (id),
                foreign key (artisttype_id) references artisttypes (id)
            );
        ").execute(db).await?;
        Ok(())
    }
    async fn idx_albumartistsunique(db: &SqlitePool) -> Result<(), SqlxError> {
        query("
            create unique index if not exists albumartistsunique
            on albumartists (
                artist_id, album_id, artisttype_id
            );
        ").execute(db).await?;
        Ok(())
    }
    async fn tbl_songs(db: &SqlitePool) -> Result<(), SqlxError> {
        query("
            create table if not exists songs (
                id integer not null primary key autoincrement,
                name text not null,
                blurb text null,
                created_date text not null,
                last_edit_date text not null
            );
        ").execute(db).await?;
        Ok(())
    }
    async fn tbl_files(db: &SqlitePool) -> Result<(), SqlxError> {
        query("
            create table if not exists files (
                id integer not null primary key autoincrement,
                file_blob blob not null,
                mime_type text not null,
                created_date text not null,
                last_edit_date text not null
            );
        ").execute(db).await?;
        Ok(())
    }
    async fn tbl_albumtracks(db: &SqlitePool) -> Result<(), SqlxError> {
        query("
            create table if not exists albumtracks (
                id integer not null primary key autoincrement,
                album_id integer not null,
                song_id integer not null,
                file_id integer null,
                track_number integer not null,
                version text null,
                created_date text not null,
                last_edit_date text not null,
                foreign key (album_id) references albums (id),
                foreign key (song_id) references songs (id),
                foreign key (file_id) references files (id)
            );
        ").execute(db).await?;
        Ok(())
    }
    async fn idx_albumtracksunique(db: &SqlitePool) -> Result<(), SqlxError> {
        query("
            create unique index if not exists albumtracksunique
            on albumtracks (
                album_id,
                song_id
            );
        ").execute(db).await?;
        Ok(())
    }
    async fn tbl_albumtrackartists(db: &SqlitePool) -> Result<(), SqlxError> {
        query("
            create table if not exists albumtrackartists (
                id integer not null primary key autoincrement,
                artist_id integer not null,
                albumtrack_id integer not null,
                artisttype_id integer not null,
                created_date text not null,
                last_edit_date text not null,
                foreign key (artist_id) references artists (id),
                foreign key (albumtrack_id) references albumtracks (id),
                foreign key (artisttype_id) references artisttypes (id)
            );
        ").execute(db).await?;
        Ok(())
    }
    async fn idx_albumtrackartistsunique(db: &SqlitePool) -> Result<(), SqlxError> {
        query("
            create unique index if not exists albumtrackartistsunique
            on albumtrackartists (
                artist_id, albumtrack_id, artisttype_id
            )
        ").execute(db).await?;
        Ok(())
    }
    pub async fn migrate(db: &SqlitePool) -> Result<(), SqlxError> {
        Self::tbl_artists(db).await?;
        Self::idx_artistsname(db).await?;
        Self::tbl_artisttypes(db).await?;
        Self::idx_artisttypesname(db).await?;
        Self::tbl_albumtypes(db).await?;
        Self::idx_albumtypesname(db).await?;
        Self::tbl_files(db).await?;
        Self::tbl_albums(db).await?;
        Self::tbl_albumartists(db).await?;
        Self::idx_albumartistsunique(db).await?;
        Self::tbl_songs(db).await?;
        Self::tbl_albumtracks(db).await?;
        Self::idx_albumtracksunique(db).await?;
        Self::tbl_albumtrackartists(db).await?;
        Self::idx_albumtrackartistsunique(db).await?;
        Ok(())
    }
}
