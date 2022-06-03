use {
    crate::{
        albumtype::AlbumType,
        migrator::DecibelMigrator,
        test_utils::db_test,
    },
    sqlx::SqlitePool,
};
async fn do_ins_album_type(db: SqlitePool) {
    DecibelMigrator::migrate(&db).await.unwrap();
    let ins_1 = AlbumType::always(&db, "SomeAlbumType").await;
    assert!(ins_1.is_ok());
}
#[async_std::test]
async fn ins_album_type() {
    db_test(do_ins_album_type).await;
}
async fn do_set_albumtype_desc(db: SqlitePool) {
    DecibelMigrator::migrate(&db).await.unwrap();
    let mut ins_1 = AlbumType::always(&db, "SomeAlbumType").await.unwrap();
    ins_1.set_description(&db, "A full release").await.unwrap();
    assert_eq!(ins_1.get_description(), Some("A full release".to_string()));
}
#[async_std::test]
async fn set_albumtype_desc() {
    db_test(do_set_albumtype_desc).await;
}
async fn do_dup_album_type_lookup(db: SqlitePool) {
    DecibelMigrator::migrate(&db).await.unwrap();
    let dup_1 = AlbumType::always(&db, "SomeAlbumType").await;
    let dup_2 = AlbumType::always(&db, "SomeAlbumType").await;
    assert!(dup_1.is_ok());
    assert!(dup_2.is_ok());
}
#[async_std::test]
async fn dup_album_type_lkup() {
    db_test(do_dup_album_type_lookup).await;
}
async fn do_dup_album_type_insert(db: SqlitePool) {
    DecibelMigrator::migrate(&db).await.unwrap();
    let dup_1 = AlbumType::always(&db, "SomeAlbumType").await;
    let dup_2 = AlbumType::insert(&db, "SomeAlbumType").await;
    assert!(dup_1.is_ok());
    assert!(dup_2.is_err());
}
#[async_std::test]
async fn dup_album_type_ins() {
    db_test(do_dup_album_type_insert).await;
}
