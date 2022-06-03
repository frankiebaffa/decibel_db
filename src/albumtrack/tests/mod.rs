use {
    crate::{
        album::Album,
        albumtype::AlbumType,
        albumtrack::AlbumTrack,
        file::File,
        migrator::DecibelMigrator,
        song::Song,
        test_utils::db_test,
    },
    sqlx::SqlitePool,
};
async fn do_ins_albumtrack(db: SqlitePool) {
    DecibelMigrator::migrate(&db).await.unwrap();
    let albumtype = AlbumType::always(&db, "LP").await.unwrap();
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
    let albumtype = AlbumType::always(&db, "LP").await.unwrap();
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
    let albumtype = AlbumType::always(&db, "LP").await.unwrap();
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
