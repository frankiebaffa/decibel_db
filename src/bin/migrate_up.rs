use migaton::Migrator;
use decibel_db::{constants, db::Db, context::Context};
fn main() {
    match dotenv::dotenv() {
        Ok(_) => {},
        Err(_) => {},
    };
    let db_path = match std::env::var(constants::DECIBEL_DB_PATH_KEY) {
        Ok(db_path) => db_path,
        Err(e) => panic!("{}", e),
    };
    let mig_path = match std::env::var(constants::DECIBEL_MIGRATION_PATH_KEY) {
        Ok(mig_path) => mig_path,
        Err(e) => panic!("{}", e),
    };
    let ctx = Context::init().unwrap();
    let mut mem_c = rusqlite::Connection::open(":memory:").unwrap();
    Db::attach_temp_db(&mut mem_c, &ctx).unwrap();
    let mut c = rusqlite::Connection::open(":memory:").unwrap();
    Db::attach_db(&mut c, &ctx).unwrap();
    let skips = match Migrator::do_up(&mut mem_c, &mut c, &mig_path) {
        Ok(res) => res,
        Err(e) => panic!("{}", e),
    };
    println!("{} migrations were skipped", skips);
}
