use {
    crate::{
        artist::Artist,
        migrator::DecibelMigrator,
        test_utils::db_test,
    },
    sqlx::SqlitePool,
};
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
async fn do_get_all_artists(db: SqlitePool) {
    DecibelMigrator::migrate(&db).await.unwrap();
    Artist::insert(&db, "Artist One").await.unwrap();
    Artist::insert(&db, "Artist Two").await.unwrap();
    Artist::insert(&db, "Artist Three").await.unwrap();
    let artists = Artist::get_all(&db).await.unwrap();
    assert_eq!(artists.len(), 3);
}
#[async_std::test]
async fn get_all_artists() {
    db_test(do_get_all_artists).await;
}
async fn do_delete_artist(db: SqlitePool) {
    DecibelMigrator::migrate(&db).await.unwrap();
    let artist = Artist::insert(&db, "Artist One").await.unwrap();
    let id = artist.id.clone();
    artist.delete(&db).await.unwrap();
    let chk_deleted = Artist::lookup_by_id(&db, id).await;
    assert!(chk_deleted.is_err());
}
#[async_std::test]
async fn delete_artist() {
    db_test(do_delete_artist).await;
}
