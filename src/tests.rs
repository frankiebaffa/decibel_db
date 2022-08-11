use {
    crate::{
        migrator::DecibelMigrator,
        test_utils::db_test,
    },
    sqlx::SqlitePool,
};
async fn do_migrate(db: SqlitePool) {
    DecibelMigrator::migrate(&db).await.unwrap();
}
#[async_std::test]
async fn migrate_up() {
    db_test(do_migrate).await;
}
