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
                        active integer not null default 1,
                        created_date text not null,
                        last_edit_date text not null
                    );
            ").execute(db).await?;
        }
        Ok(())
    }
    async fn ins_artisttypes(db: &SqlitePool) -> Result<(), SqlxError> {
        let chk = query_as::<_, (i64,)>("
            select count(*)
            from artisttypes
            limit 1;
        ").fetch_one(db).await? .0 > 0;
        if !chk {
            query("
                insert into artisttypes
                    (
                        name,
                        descriptor
                    )
                select
                    'Performer',
                    'Performed by'
                union all
                select
                    'Writer',
                    'Written by'
                union all
                select
                    'Composer',
                    'Composed by'
                union all
                select
                    'Producer',
                    'Produced by'
                union all
                select
                    'Feature',
                    'Featuring';
            ").execute(db).await?;
        }
        Ok(())
    }
    async fn idx_artisttypesname(db: &SqlitePool) -> Result<(), SqlxError> {
        let chk = query_as::<_, (i64,)>("
            select count(name)
            from sqlite_master
            where type = 'index'
            and name = 'artisttypesnameunique'
        ").fetch_one(db).await? .0 > 0;
        if !chk {
            query("
                create unique index artisttypesnameunique
                on artisttypes (name)
                where active = 1;
            ").execute(db).await?;
        }
        Ok(())
    }
    async fn tbl_albumtypes(db: &SqlitePool) -> Result<(), SqlxError> {
        let chk = query_as::<_, (i64,)>("
            select count(name)
            from sqlite_master
            where type = 'table'
            and name = 'albumtypes';
        ").fetch_one(db).await? .0 > 0;
        if !chk {
            query("
                create table albumtypes (
                    id integer not null primary key autoincrement,
                    name text not null,
                    description text null,
                    active integer not null default 1,
                    created_date text not null,
                    last_edit_date text not null
                );
            ").execute(db).await?;
        }
        Ok(())
    }
    async fn idx_albumtypesname(db: &SqlitePool) -> Result<(), SqlxError> {
        let chk = query_as::<_, (i64,)>("
            select count(name)
            from sqlite_master
            where type = 'index'
            and name = 'albumtypesnameunique';
        ").fetch_one(db).await? .0 > 0;
        if !chk {
            query("
                create unique index albumtypesnameunique
                on albumtypes (name)
                where active = 1;
            ").execute(db).await?;
        }
        Ok(())
    }
    async fn ins_albumtypes(db: &SqlitePool) -> Result<(), SqlxError> {
        let chk = query_as::<_, (i64,)>("
            select count(Name)
            from albumtypes;
        ").fetch_one(db).await? .0 > 0;
        if !chk {
            query("
                insert into albumtypes (
                    name,
                    description
                )
                select
                    'LP',
                    'A long play'
                union all
                select
                    'EP',
                    'An extended play'
                union all
                select
                    'Single',
                    'A single release';
                select
                    'Mixtape',
                    'An unofficial release'
                union all
                select
                    'Compilation',
                    'A release with varying track artists';
            ").execute(db).await?;
        }
        Ok(())
    }
    async fn tbl_albums(db: &SqlitePool) -> Result<(), SqlxError> {
        let chk = query_as::<_, (i64,)>("
            select count(name)
            from sqlite_master
            where type = 'table'
            and name = 'albums';
        ").fetch_one(db).await? .0 > 0;
        if !chk {
            query("
                create table albums (
                    id integer not null primary key autoincrement,
                    albumtype_id integer not null,
                    name text not null,
                    blurb text null,
                    active integer not null default 1,
                    cover_id integer null,
                    release_date text null,
                    created_date text not null,
                    last_edit_date text not null,
                    foreign key (albumtype_id) references albumtypes(id),
                    foreign key (cover_id) references files (id)
                );
            ").execute(db).await?;
        }
        Ok(())
    }
    async fn tbl_albumartists(db: &SqlitePool) -> Result<(), SqlxError> {
        let chk = query_as::<_, (i64,)>("
            select count(name)
            from sqlite_master
            where type = 'table'
            and name = 'albumartists';
        ").fetch_one(db).await? .0 > 0;
        if !chk {
            query("
                create table albumartists (
                    id integer not null primary key autoincrement,
                    artist_id integer not null,
                    album_id integer not null,
                    artisttype_id integer not null,
                    active integer not null default 1,
                    created_date text not null,
                    last_edit_date text not null,
                    foreign key (artist_id) references artists (id),
                    foreign key (album_id) references albums (id),
                    foreign key (artisttype_id) references artisttypes (id)
                );
            ").execute(db).await?;
        }
        Ok(())
    }
    async fn idx_albumartistsunique(db: &SqlitePool) -> Result<(), SqlxError> {
        let chk = query_as::<_, (i64,)>("
            select count(name)
            from sqlite_master
            where type = 'index'
            and name = 'albumartistsunique';
        ").fetch_one(db).await? .0 > 0;
        if !chk {
            query("
                create unique index albumartistsunique
                on albumartists (
                    artist_id, album_id, artisttype_id
                )
                where active = 1;
            ").execute(db).await?;
        }
        Ok(())
    }
    async fn tbl_songs(db: &SqlitePool) -> Result<(), SqlxError> {
        let chk = query_as::<_, (i64,)>("
            select count(name)
            from sqlite_master
            where type = 'table'
            and name = 'songs';
        ").fetch_one(db).await? .0 > 0;
        if !chk {
            query("
                create table songs (
                    id integer not null primary key autoincrement,
                    name text not null,
                    blurb text null,
                    active integer not null default 1,
                    created_date text not null,
                    last_edit_date text not null
                );
            ").execute(db).await?;
        }
        Ok(())
    }
    async fn tbl_files(db: &SqlitePool) -> Result<(), SqlxError> {
        let chk = query_as::<_, (i64,)>("
            select count(name)
            from sqlite_master
            where type = 'table'
            and name = 'files';
        ").fetch_one(db).await?.0 > 0;
        if !chk {
            query("
                create table files (
                    id integer not null primary key autoincrement,
                    file_blob blob not null,
                    mime_type text not null,
                    active integer not null default 1,
                    created_date text not null,
                    last_edit_date text not null
                );
            ").execute(db).await?;
        }
        Ok(())
    }
    async fn tbl_albumtracks(db: &SqlitePool) -> Result<(), SqlxError> {
        let chk = query_as::<_, (i64,)>("
            select count(name)
            from sqlite_master
            where type = 'table'
            and name = 'albumtracks';
        ").fetch_one(db).await? .0 > 0;
        if !chk {
            query("
                create table albumtracks (
                    id integer not null primary key autoincrement,
                    album_id integer not null,
                    song_id integer not null,
                    file_id integer null,
                    track_number integer not null,
                    version text null,
                    active integer not null default 1,
                    created_date text not null,
                    last_edit_date text not null,
                    foreign key (album_id) references albums (id),
                    foreign key (song_id) references songs (id),
                    foreign key (file_id) references files (id)
                );
            ").execute(db).await?;
        }
        Ok(())
    }
    async fn idx_albumtracksunique(db: &SqlitePool) -> Result<(), SqlxError> {
        let chk = query_as::<_, (i64,)>("
            select count(name)
            from sqlite_master
            where type = 'index'
            and name = 'albumtracksunique';
        ").fetch_one(db).await? .0 > 0;
        if !chk {
            query("
                create unique index albumtracksunique
                on albumtracks (
                    album_id,
                    song_id
                )
                where active = 1;
            ").execute(db).await?;
        }
        Ok(())
    }
    async fn tbl_albumtrackartists(db: &SqlitePool) -> Result<(), SqlxError> {
        let chk = query_as::<_, (i64,)>("
            select count(name)
            from sqlite_master
            where type = 'table'
            and name = 'albumtrackartists';
        ").fetch_one(db).await? .0 > 0;
        if !chk {
            query("
                create table albumtrackartists (
                    id integer not null primary key autoincrement,
                    artist_id integer not null,
                    albumtrack_id integer not null,
                    artisttype_id integer not null,
                    active integer not null default 1,
                    created_date text not null,
                    last_edit_date text not null,
                    foreign key (artist_id) references artists (id),
                    foreign key (albumtrack_id) references albumtracks (id),
                    foreign key (artisttype_id) references artisttypes (id)
                );
            ").execute(db).await?;
        }
        Ok(())
    }
    async fn idx_albumtrackartistsunique(db: &SqlitePool) -> Result<(), SqlxError> {
        let chk = query_as::<_, (i64,)>("
            select count(name)
            from sqlite_master
            where type = 'index'
            and name = 'albumtrackartistsunique';
        ").fetch_one(db).await? .0 > 0;
        if !chk {
            query("
                create unique index albumtrackartistsunique
                on albumtrackartists (
                    artist_id, albumtrack_id, artisttype_id
                )
                where active = 1;
            ").execute(db).await?;
        }
        Ok(())
    }
    pub async fn migrate(db: &SqlitePool) -> Result<(), SqlxError> {
        Self::tbl_artists(db).await?;
        Self::idx_artistsname(db).await?;
        Self::tbl_artisttypes(db).await?;
        Self::ins_artisttypes(db).await?;
        Self::idx_artisttypesname(db).await?;
        Self::tbl_albumtypes(db).await?;
        Self::idx_albumtypesname(db).await?;
        Self::ins_albumtypes(db).await?;
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
