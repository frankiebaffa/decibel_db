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
    let _ = AlbumType::insert(&db, "EP").await.unwrap();
    let ins_1 = AlbumType::lookup_by_name(&db, "EP").await
        .unwrap()
        .unwrap();
    assert!(ins_1.get_name().eq(&"EP"));
}
#[async_std::test]
async fn ins_album_type() {
    db_test(do_ins_album_type).await;
}
async fn do_set_albumtype_desc(db: SqlitePool) {
    DecibelMigrator::migrate(&db).await.unwrap();
    let type_id = AlbumType::insert(&db, "LP").await.unwrap();
    let mut ins_1 = AlbumType::lookup_by_id(&db, type_id).await.unwrap()
        .unwrap();
    ins_1.set_description(&db, "A full release").await.unwrap();
    assert_eq!(ins_1.get_description(), Some("A full release".to_string()));
}
#[async_std::test]
async fn set_albumtype_desc() {
    db_test(do_set_albumtype_desc).await;
}
async fn do_dup_album_type_lookup(db: SqlitePool) {
    DecibelMigrator::migrate(&db).await.unwrap();
    let type_1_id = AlbumType::insert(&db, "SomeAlbumType").await;
    let type_2_id = AlbumType::insert(&db, "SomeAlbumType").await;
    assert!(type_1_id.is_ok());
    assert!(type_2_id.is_err());
}
#[async_std::test]
async fn dup_album_type_lkup() {
    db_test(do_dup_album_type_lookup).await;
}
