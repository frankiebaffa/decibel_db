use {
    crate::{
        album::Album,
        albumtype::AlbumType,
        albumartist::AlbumArtist,
        albumtrack::AlbumTrack,
        artist::Artist,
        artisttype::ArtistType,
        file::File,
        migrator::DecibelMigrator,
        song::Song,
    },
    sqlx::SqlitePool,
    std::{
        future::Future,
        path::PathBuf,
    },
    uuid::Uuid,
};
fn get_file_name() -> String {
    let uuid = Uuid::new_v4();
    let name = format!(
        "./test_dbs/rusters_{}.db",
        uuid.as_hyphenated()
    );
    name
}
fn create_db_file<'a>(name: &'a str) {
    let db_path = PathBuf::from(name);
    std::fs::File::create(db_path)
        .unwrap();
}
fn delete_db_file<'a>(name: &'a str) {
    let db_path = PathBuf::from(name);
    std::fs::remove_file(db_path)
        .unwrap();
    let db_wal = PathBuf::from(format!("{}-wal", name));
    std::fs::remove_file(db_wal)
        .unwrap();
    let db_shm = PathBuf::from(format!("{}-shm", name));
    std::fs::remove_file(db_shm)
        .unwrap();
}
async fn get_db<'a>(name: &'a str) -> SqlitePool {
    let path = format!("sqlite://{}", name);
    SqlitePool::connect(&path).await.unwrap()
}
async fn db_test<T, Fut>(function: T)
where
    T: Fn(SqlitePool) -> Fut,
    Fut: Future,
{
    let db_name = get_file_name();
    create_db_file(&db_name);
    let db = get_db(&db_name).await;
    function(db).await;
    delete_db_file(&db_name);
}
async fn do_migrate(db: SqlitePool) {
    DecibelMigrator::migrate(&db).await.unwrap();
}
#[async_std::test]
async fn migrate_up() {
    db_test(do_migrate).await;
}
async fn do_insert_artist(db: SqlitePool) {
    DecibelMigrator::migrate(&db).await.unwrap();
    let artist = Artist::insert(&db, "Test Artist")
        .await
        .unwrap();
    assert_eq!(artist.get_name(), "Test Artist".to_string());
}
#[async_std::test]
async fn ins_artist() {
    db_test(do_insert_artist).await;
}
async fn do_dup_ins_artist(db: SqlitePool) {
    DecibelMigrator::migrate(&db).await.unwrap();
    let artist_1 = Artist::insert(&db, "Test Artist").await;
    let artist_2 = Artist::insert(&db, "Test Artist").await;
    assert!(artist_1.is_ok());
    assert!(artist_2.is_err());
}
#[async_std::test]
async fn dup_ins_artist() {
    db_test(do_dup_ins_artist).await;
}
async fn do_insert_artist_type(db: SqlitePool) {
    DecibelMigrator::migrate(&db).await.unwrap();
    let artist_type = ArtistType::lookup(&db, "Composer", Some("Composed By")).await
        .unwrap();
    assert_eq!(artist_type.get_name(), "Composer");
}
#[async_std::test]
async fn ins_artist_type() {
    db_test(do_insert_artist_type).await;
}
async fn do_dup_artist_type(db: SqlitePool) {
    DecibelMigrator::migrate(&db).await.unwrap();
    let type_1 = ArtistType::insert(&db, "SomeArtist", "Some Artist").await;
    let type_2 = ArtistType::insert(&db, "SomeArtist", "Some Artist").await;
    assert!(type_1.is_ok());
    assert!(type_2.is_err());
}
#[async_std::test]
async fn dup_artist_type() {
    db_test(do_dup_artist_type).await;
}
async fn do_ins_album_type(db: SqlitePool) {
    DecibelMigrator::migrate(&db).await.unwrap();
    let ins_1 = AlbumType::lookup(&db, "SomeAlbumType", Some("Some album type")).await;
    assert!(ins_1.is_ok());
}
#[async_std::test]
async fn ins_album_type() {
    db_test(do_ins_album_type).await;
}
async fn do_dup_album_type_lookup(db: SqlitePool) {
    DecibelMigrator::migrate(&db).await.unwrap();
    let dup_1 = AlbumType::lookup(&db, "SomeAlbumType", Some("Some album type")).await;
    let dup_2 = AlbumType::lookup(&db, "SomeAlbumType", Some("Some album type")).await;
    assert!(dup_1.is_ok());
    assert!(dup_2.is_ok());
}
#[async_std::test]
async fn dup_album_type_lkup() {
    db_test(do_dup_album_type_lookup).await;
}
async fn do_dup_album_type_insert(db: SqlitePool) {
    DecibelMigrator::migrate(&db).await.unwrap();
    let dup_1 = AlbumType::lookup(&db, "SomeAlbumType", Some("Some album type")).await;
    let dup_2 = AlbumType::insert(&db, "SomeAlbumType", "Some album type").await;
    assert!(dup_1.is_ok());
    assert!(dup_2.is_err());
}
#[async_std::test]
async fn dup_album_type_ins() {
    db_test(do_dup_album_type_insert).await;
}
async fn do_ins_song(db: SqlitePool) {
    DecibelMigrator::migrate(&db).await.unwrap();
    let song = Song::insert(&db, "A Song").await;
    assert!(song.is_ok());
}
#[async_std::test]
async fn ins_song() {
    db_test(do_ins_song).await;
}
async fn do_set_song_blurb(db: SqlitePool) {
    DecibelMigrator::migrate(&db).await.unwrap();
    let mut song = Song::insert(&db, "A Song").await.unwrap();
    let set_res = song.set_blurb(&db, "Hello, World!").await;
    assert!(set_res.is_ok());
}
#[async_std::test]
async fn set_song_blurb() {
    db_test(do_set_song_blurb).await;
}
async fn do_ins_file(db: SqlitePool) {
    DecibelMigrator::migrate(&db).await.unwrap();
    let blob = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));
    let file = File::insert(&db, blob, "text/plain").await.unwrap();
    assert!(file.get_file_blob().len() > 0);
    assert!(file.get_mime_type().eq("text/plain"));
}
#[async_std::test]
async fn ins_file() {
    db_test(do_ins_file).await;
}
async fn do_ins_album(db: SqlitePool) {
    DecibelMigrator::migrate(&db).await.unwrap();
    let albumtype = AlbumType::lookup(&db, "SomeAlbumType", Some("Some album type")).await.unwrap();
    let album = Album::insert(&db, &albumtype, "Album One").await.unwrap();
    assert_eq!(album.get_name(), "Album One");
    assert_eq!(album.get_albumtype_id(), albumtype.get_id());
    assert_eq!(album.get_cover_id(), None);
}
#[async_std::test]
async fn ins_album() {
    db_test(do_ins_album).await;
}
async fn do_set_album_cover(db: SqlitePool) {
    DecibelMigrator::migrate(&db).await.unwrap();
    let blob = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));
    let file = File::insert(&db, blob, "text/plain").await.unwrap();
    let albumtype = AlbumType::lookup(&db, "SomeAlbumType", Some("Some album type")).await.unwrap();
    let mut album = Album::insert(&db, &albumtype, "Album One").await.unwrap();
    assert_eq!(album.get_name(), "Album One");
    assert_eq!(album.get_albumtype_id(), albumtype.get_id());
    album.set_cover(&db, &file).await.unwrap();
    assert_eq!(album.get_cover_id(), Some(file.get_id()));
}
#[async_std::test]
async fn set_album_cover() {
    db_test(do_set_album_cover).await;
}
async fn do_ins_albumartist(db: SqlitePool) {
    DecibelMigrator::migrate(&db).await.unwrap();
    let artist = Artist::insert(&db, "Artist One").await.unwrap();
    let albumtype = AlbumType::lookup(&db, "LP", Some("A full album release"))
        .await
        .unwrap();
    let album = Album::insert(&db, &albumtype, "Album One").await.unwrap();
    let artisttype = ArtistType::lookup(&db, "Writer", Some("The primary writer of the album"))
        .await
        .unwrap();
    let albumartist = AlbumArtist::insert(&db, &artist, &album, &artisttype)
        .await
        .unwrap();
    assert_eq!(albumartist.get_album_id(), album.get_id());
    assert_eq!(albumartist.get_artist_id(), artist.get_id());
    assert_eq!(albumartist.get_artisttype_id(), artisttype.get_id());
}
#[async_std::test]
async fn ins_albumartist() {
    db_test(do_ins_albumartist).await;
}
async fn do_ins_albumtrack(db: SqlitePool) {
    DecibelMigrator::migrate(&db).await.unwrap();
    let albumtype = AlbumType::lookup(&db, "LP", None).await.unwrap();
    let album = Album::insert(&db, &albumtype, "Album One").await.unwrap();
    let song = Song::insert(&db, "Song One").await.unwrap();
    let albumtrack = AlbumTrack::insert(&db, &album, &song, 1).await.unwrap();
    assert_eq!(albumtrack.get_album_id(), album.get_id());
    assert_eq!(albumtrack.get_song_id(), song.get_id());
}
#[async_std::test]
async fn ins_albumtrack() {
    db_test(do_ins_albumtrack).await;
}
async fn do_set_albumtrack_version(db: SqlitePool) {
    DecibelMigrator::migrate(&db).await.unwrap();
    let albumtype = AlbumType::lookup(&db, "LP", None).await.unwrap();
    let album = Album::insert(&db, &albumtype, "Album One").await.unwrap();
    let song = Song::insert(&db, "Song One").await.unwrap();
    let mut albumtrack = AlbumTrack::insert(&db, &album, &song, 1).await.unwrap();
    albumtrack.set_version(&db, "Live Version").await.unwrap();
    let version = albumtrack.get_version().unwrap();
    assert_eq!(version, "Live Version");
}
#[async_std::test]
async fn set_albumtrack_version() {
    db_test(do_set_albumtrack_version).await;
}
async fn do_set_albumtrack_file(db: SqlitePool) {
    DecibelMigrator::migrate(&db).await.unwrap();
    let albumtype = AlbumType::lookup(&db, "LP", None).await.unwrap();
    let album = Album::insert(&db, &albumtype, "Album One").await.unwrap();
    let song = Song::insert(&db, "Song One").await.unwrap();
    let file_b = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));
    let file = File::insert(&db, file_b, "text/plain").await.unwrap();
    let mut albumtrack = AlbumTrack::insert(&db, &album, &song, 1).await.unwrap();
    albumtrack.set_file(&db, &file).await.unwrap();
    let file_id = albumtrack.get_file_id().unwrap();
    assert_eq!(file.get_id(), file_id);
}
#[async_std::test]
async fn set_albumtrack_file() {
    db_test(do_set_albumtrack_file).await;
}
