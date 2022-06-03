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
async fn do_get_files_from_albumtracks(db: SqlitePool) {
    DecibelMigrator::migrate(&db).await.unwrap();
    let albumtype = AlbumType::always(&db, "LP").await.unwrap();
    let album = Album::insert(&db, &albumtype, "Album One").await.unwrap();
    let song_1 = Song::insert(&db, "Song One").await.unwrap();
    let file_b = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));
    let file_1 = File::insert(&db, file_b, "text/plain").await.unwrap();
    let mut albumtrack_1 = AlbumTrack::insert(&db, &album, &song_1, 1).await.unwrap();
    albumtrack_1.set_file(&db, &file_1).await.unwrap();
    let file_2 = File::insert(&db, file_b, "text/plain").await.unwrap();
    let song_2 = Song::insert(&db, "Song Two").await.unwrap();
    let mut albumtrack_2 = AlbumTrack::insert(&db, &album, &song_2, 2).await.unwrap();
    albumtrack_2.set_file(&db, &file_2).await.unwrap();
    let albumtracks = AlbumTrack::from_album(&db, &album).await.unwrap();
    let files = File::from_albumtracks(&db, &albumtracks).await.unwrap();
    assert_eq!(files.len(), 2);
}
#[async_std::test]
async fn get_files_from_albumtracks() {
    db_test(do_get_files_from_albumtracks).await;
}
