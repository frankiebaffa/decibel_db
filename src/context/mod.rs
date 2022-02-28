use crate::constants;
use std::env;
pub struct Context {
    db_path: String,
    db_name: String,
}
impl Context {
    pub fn init() -> Result<Context, env::VarError> {
        match dotenv::dotenv() {
            Ok(_) => {},
            Err(_) => {},
        }
        let db_path = match env::var(constants::DECIBEL_DB_PATH_KEY) {
            Ok(db_path) => db_path,
            Err(e) => return Err(e),
        };
        return Ok(Context {
            db_path,
            db_name: String::from(constants::DECIBEL_DB_NAME),
        });
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
}
