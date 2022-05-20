use {
    crate::migrator::DecibelMigrator,
    sqlx::SqlitePool,
    std::path::PathBuf,
};
fn get_file_name() -> String {
    let mut num = 0;
    let mut name;
    loop {
        name = format!(
            "./test_dbs/rusters_{}.db",
            num
        );
        let chk = PathBuf::from(&name);
        if !chk.exists() {
            break;
        }
        num = num + 1;
    }
    name
}
fn create_db_file_if_not_exist<'a>(name: &'a str) {
    let db_path = PathBuf::from(name);
    if !db_path.exists() {
        let file_res = std::fs::File::create(db_path);
        file_res.unwrap();
    }
}
fn delete_db_file_if_exists<'a>(name: &'a str) {
    let db_path = PathBuf::from(name);
    if db_path.exists() {
        let rem_res = std::fs::remove_file(db_path);
        rem_res.unwrap();
    }
    let db_wal = PathBuf::from(format!("{}-wal", name));
    if db_wal.exists() {
        let rem_res = std::fs::remove_file(db_wal);
        rem_res.unwrap();
    }
    let db_shm = PathBuf::from(format!("{}-shm", name));
    if db_shm.exists() {
        let rem_res = std::fs::remove_file(db_shm);
        rem_res.unwrap();
    }
}
async fn get_db<'a>(name: &'a str) -> SqlitePool {
    let path = format!("sqlite://{}", name);
    SqlitePool::connect(&path).await.unwrap()
}
#[async_std::test]
async fn migrate_up() {
    let db_name = get_file_name();
    create_db_file_if_not_exist(&db_name);
    let db = get_db(&db_name).await;
    DecibelMigrator::migrate(&db).await.unwrap();
    delete_db_file_if_exists(&db_name);
}

