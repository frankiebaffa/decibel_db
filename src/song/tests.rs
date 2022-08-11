use {
    crate::{
        migrator::DecibelMigrator,
        song::Song,
        test_utils::db_test,
    },
    sqlx::SqlitePool,
};
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
