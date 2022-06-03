use {
    crate::{
        albumtype::AlbumType,
        album::Album,
        file::File,
        migrator::DecibelMigrator,
        test_utils::db_test,
    },
    sqlx::SqlitePool,
};
async fn do_ins_album(db: SqlitePool) {
    DecibelMigrator::migrate(&db).await.unwrap();
    let albumtype = AlbumType::always(&db, "SomeAlbumType").await.unwrap();
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
    let albumtype = AlbumType::always(&db, "SomeAlbumType").await.unwrap();
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
