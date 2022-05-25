use {
    crate::{
        artist::Artist,
        migrator::DecibelMigrator,
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
    let artist = Artist::insert(&db, "Test Artist".to_string())
        .await
        .unwrap();
    assert_eq!(artist.get_name(), "Test Artist".to_string());
}
#[async_std::test]
async fn ins_artist() {
    db_test(do_insert_artist).await;
}
