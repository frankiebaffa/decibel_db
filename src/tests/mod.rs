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
fn get_db_ctx() -> Database {
    let mut db = Database::init();
    db.context.attach_dbs();
    db
}
fn migrate_up() {
    DecibelMigrator::migrate_up::<Database>(None);
}
fn migrate_down() {
    DecibelMigrator::migrate_down::<Database>(None);
}
#[test]
#[serial]
fn migrations() {
    migrate_up();
    migrate_down();
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
    migrate_up();
    let mut db = get_db_ctx();
    insert_new_artist(&mut db);
    migrate_down();
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
    migrate_up();
    let mut db = get_db_ctx();
    get_album_type_by_name(&mut db);
    migrate_down();
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
    migrate_up();
    let mut db = get_db_ctx();
    let at = get_album_type_by_name(&mut db);
    insert_new_album(&mut db, &at);
    migrate_down();
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
    migrate_up();
    let mut db = get_db_ctx();
    get_artist_type_by_name(&mut db);
    migrate_down();
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
    migrate_up();
    let mut db = get_db_ctx();
    let ar = insert_new_artist(&mut db);
    let at = get_album_type_by_name(&mut db);
    let al = insert_new_album(&mut db, &at);
    let art = get_artist_type_by_name(&mut db);
    insert_new_album_artist(&mut db, &ar, &al, &art);
    migrate_down();
}
