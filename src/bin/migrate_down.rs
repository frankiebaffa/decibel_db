use migaton::Migrator;
use bangers_db::constants;
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
    let mig_path = match std::env::var(constants::BANGERS_MIGRATION_PATH) {
        Ok(mig_path) => mig_path,
        Err(e) => panic!("{}", e),
    };
    let skips = match Migrator::do_down(db_path.as_str(), mig_path.as_str(), db_name.as_str()) {
        Ok(res) => res,
        Err(e) => panic!("{}", e),
    };
    println!("{} migrations were skipped", skips);
}
