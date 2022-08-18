use {
    crate::{
        artisttype::ArtistType,
        migrator::DecibelMigrator,
        test_utils::db_test,
    },
    sqlx::SqlitePool,
};
async fn do_insert_artist_type(db: SqlitePool) {
    DecibelMigrator::migrate(&db).await.unwrap();
    let _ = ArtistType::insert(&db, "Composer", "Composed By").await
        .unwrap();
    let artist_type = ArtistType::lookup_by_name(&db, "Composer")
        .await
        .unwrap()
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
