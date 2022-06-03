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
async fn do_get_all_active_artists(db: SqlitePool) {
    DecibelMigrator::migrate(&db).await.unwrap();
    Artist::insert(&db, "Artist One").await.unwrap();
    Artist::insert(&db, "Artist Two").await.unwrap();
    Artist::insert(&db, "Artist Three").await.unwrap();
    let artists = Artist::get_all_active(&db).await.unwrap();
    assert_eq!(artists.len(), 3);
}
#[async_std::test]
async fn get_all_active_artists() {
    db_test(do_get_all_active_artists).await;
}
async fn do_deactivate_artist(db: SqlitePool) {
    DecibelMigrator::migrate(&db).await.unwrap();
    let mut artist = Artist::insert(&db, "Artist One").await.unwrap();
    artist.deactivate(&db).await.unwrap();
    assert_eq!(artist.get_active(), false);
}
#[async_std::test]
async fn deactivate_artist() {
    db_test(do_deactivate_artist).await;
}
async fn do_get_all_inctive_artists(db: SqlitePool) {
    DecibelMigrator::migrate(&db).await.unwrap();
    Artist::insert(&db, "Artist One").await.unwrap()
        .deactivate(&db)
        .await
        .unwrap();
    Artist::insert(&db, "Artist Two").await.unwrap()
        .deactivate(&db)
        .await
        .unwrap();
    Artist::insert(&db, "Artist Three").await.unwrap()
        .deactivate(&db)
        .await
        .unwrap();
    let artists = Artist::get_all_inactive(&db).await.unwrap();
    assert_eq!(artists.len(), 3);
}
#[async_std::test]
async fn get_all_inactive_artists(db: SqlitePool) {
    db_test(do_get_all_inctive_artists).await;
}
