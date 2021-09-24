use migaton::Migrator;
use bangers_db::constants;
use rusqlite::Connection;
fn main() {
    match dotenv::dotenv() {
        Ok(_) => {},
        Err(_) => {},
    };
    let db_path = match std::env::var(constants::BANGERS_DB_PATH) {
        Ok(db_path) => db_path,
        Err(e) => panic!("{}", e),
    };
    let db_name = match std::env::var(constants::BANGERS_DB_NAME) {
        Ok(db_name) => db_name,
        Err(e) => panic!("{}", e),
    };
    let db_file = if db_path.ends_with("/") {
        format!("{}{}.db", db_path, db_name)
    } else {
        format!("{}/{}.db", db_path, db_name)
    };
    let mut c = match Connection::open(db_file) {
        Ok(c) => c,
        Err(e) => panic!("{}", e),
    };
    let mig_path = match std::env::var(constants::BANGERS_MIGRATION_PATH) {
        Ok(mig_path) => mig_path,
        Err(e) => panic!("{}", e),
    };
    let skips = match Migrator::do_down(&mut c, mig_path.as_str()) {
        Ok(res) => res,
        Err(e) => panic!("{}", e),
    };
    println!("{} migrations were skipped", skips);
}
