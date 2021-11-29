use {
    crate::{
        artist::Artist,
        album::Album,
        albumartist::AlbumArtist,
        albumtype::AlbumType,
        artisttype::ArtistType,
        DecibelMigrator,
    },
    migaton::traits::DoMigrations,
    serial_test::serial,
    worm::derive::WormDb,
    worm::core::{
        DbCtx,
        DbContext,
        ForeignKey,
        PrimaryKey,
        UniqueName,
        UniqueNameModel,
    },
};
#[derive(WormDb)]
#[db(var(name="DECIBELDBS"))]
struct Database {
    context: DbContext,
}
fn get_db_ctx() -> (Database, Database) {
    let mut mem_db = Database::init();
    mem_db.context.attach_temp_dbs();
    let mut db = Database::init();
    db.context.attach_dbs();
    return (mem_db, db);
}
fn migrate_up(mem_db: &mut Database, db: &mut Database) {
    DecibelMigrator::migrate_up(mem_db, db);
}
fn migrate_down(mem_db: &mut Database, db: &mut Database) {
    DecibelMigrator::migrate_down(mem_db, db);
}
#[test]
#[serial]
fn migrations() {
    let (mut mem_db, mut db) = get_db_ctx();
    migrate_up(&mut mem_db, &mut db);
    migrate_down(&mut mem_db, &mut db);
}
const ARTIST_NAME: &'static str = "Test Artist 1";
const ARTIST_BIO: &'static str = "This is a test artist.";
fn insert_new_artist(db: &mut Database) -> Artist {
    let a_res = Artist::insert_new(db, ARTIST_NAME.to_string(), Some(ARTIST_BIO.to_string()));
    assert!(a_res.is_ok());
    let a = a_res.unwrap();
    assert_eq!(a.get_name(), ARTIST_NAME);
    assert_eq!(a.get_bio(), Some(ARTIST_BIO.to_string()));
    return a;
}
#[test]
#[serial]
fn insert_artist() {
    let (mut mem_db, mut db) = get_db_ctx();
    migrate_up(&mut mem_db, &mut db);
    insert_new_artist(&mut db);
    migrate_down(&mut mem_db, &mut db);
}
const ALBUM_TYPE_NAME: &'static str = "LP";
fn get_album_type_by_name(db: &mut Database) -> AlbumType {
    let t_res = AlbumType::get_by_name(db, ALBUM_TYPE_NAME);
    assert!(t_res.is_ok());
    let t = t_res.unwrap();
    assert_eq!(t.get_name(), ALBUM_TYPE_NAME);
    return t;
}
#[test]
#[serial]
fn get_album_type() {
    let (mut mem_db, mut db) = get_db_ctx();
    migrate_up(&mut mem_db, &mut db);
    get_album_type_by_name(&mut db);
    migrate_down(&mut mem_db, &mut db);
}
const ALBUM_NAME: &'static str = "Test Album 1";
const ALBUM_BLURB: &'static str = "This is a test album.";
fn insert_new_album(db: &mut Database, at: &AlbumType) -> Album {
    let a_res = Album::insert_new(db, at.get_id(), ALBUM_NAME.to_string(), Some(ALBUM_BLURB.to_string()));
    assert!(a_res.is_ok());
    let a = a_res.unwrap();
    assert_eq!(a.get_name(), ALBUM_NAME);
    assert_eq!(a.get_blurb(), Some(ALBUM_BLURB.to_string()));
    assert_eq!(a.get_fk_value(), at.get_id());
    return a;
}
#[test]
#[serial]
fn insert_album() {
    let (mut mem_db, mut db) = get_db_ctx();
    migrate_up(&mut mem_db, &mut db);
    let at = get_album_type_by_name(&mut db);
    insert_new_album(&mut db, &at);
    migrate_down(&mut mem_db, &mut db);
}
const ARTIST_TYPE_NAME: &'static str = "Performer";
fn get_artist_type_by_name(db: &mut Database) -> ArtistType {
    let a_res = ArtistType::get_by_name(db, ARTIST_TYPE_NAME);
    assert!(a_res.is_ok());
    let a = a_res.unwrap();
    assert_eq!(a.get_name(), ARTIST_TYPE_NAME);
    return a;
}
#[test]
#[serial]
fn get_artist_type() {
    let (mut mem_db, mut db) = get_db_ctx();
    migrate_up(&mut mem_db, &mut db);
    get_artist_type_by_name(&mut db);
    migrate_down(&mut mem_db, &mut db);
}
fn insert_new_album_artist(
    db: &mut Database, ar: &Artist, al: &Album, at: &ArtistType
) -> AlbumArtist {
    let a_res = AlbumArtist::insert_new(db, ar.get_id(), al.get_id(), at.get_id());
    assert!(a_res.is_ok());
    let a = a_res.unwrap();
    assert_eq!(
        <AlbumArtist as ForeignKey<Artist>>::get_fk_value(&a),
        ar.get_id()
    );
    assert_eq!(
        <AlbumArtist as ForeignKey<Album>>::get_fk_value(&a),
        al.get_id()
    );
    assert_eq!(
        <AlbumArtist as ForeignKey<ArtistType>>::get_fk_value(&a),
        at.get_id()
    );
    return a;
}
#[test]
#[serial]
fn insert_album_artist() {
    let (mut mem_db, mut db) = get_db_ctx();
    migrate_up(&mut mem_db, &mut db);
    let ar = insert_new_artist(&mut db);
    let at = get_album_type_by_name(&mut db);
    let al = insert_new_album(&mut db, &at);
    let art = get_artist_type_by_name(&mut db);
    insert_new_album_artist(&mut db, &ar, &al, &art);
    migrate_down(&mut mem_db, &mut db);
}
