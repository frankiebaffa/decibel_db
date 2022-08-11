use {
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
        "{}/test_dbs/decibel_{}.db",
        env!("CARGO_MANIFEST_DIR"),
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
pub async fn db_test<T, Fut>(function: T)
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
