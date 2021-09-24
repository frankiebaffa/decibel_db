use crate::constants;
use std::env;
pub struct Context {
    db_path: String,
    db_name: String,
    migration_path: String,
}
impl Context {
    pub fn init() -> Result<Context, env::VarError> {
        match dotenv::dotenv() {
            Ok(_) => {},
            Err(_) => {},
        }
        let db_path = match env::var(constants::BANGERS_DB_PATH) {
            Ok(db_path) => db_path,
            Err(e) => return Err(e),
        };
        let db_name = match env::var(constants::BANGERS_DB_NAME) {
            Ok(db_name) => db_name,
            Err(e) => return Err(e),
        };
        let migration_path = match env::var(constants::BANGERS_MIGRATION_PATH) {
            Ok(migration_path) => migration_path,
            Err(e) => return Err(e),
        };
        return Ok(Context { db_path, db_name, migration_path });
    }
    pub fn get_path_to_db(&self) -> String {
        return if self.db_path.ends_with("/") {
            format!("{}{}", self.db_path, self.db_name)
        } else {
            format!("{}/{}", self.db_path, self.db_name)
        }
    }
    pub fn get_db_path(&self) -> &str {
        return &self.db_path;
    }
    pub fn get_db_name(&self) -> &str {
        return &self.db_name;
    }
    pub fn get_migration_path(&self) -> &str {
        return &self.migration_path;
    }
}
