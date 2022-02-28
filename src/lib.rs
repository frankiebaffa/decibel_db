pub mod album;
pub mod albumartist;
pub mod albumtrackartist;
pub mod albumtrack;
pub mod albumtype;
pub mod artist;
pub mod artisttype;
pub mod constants;
pub mod context;
pub mod error;
pub mod file;
pub mod song;
#[cfg(test)]
mod tests;
use migaton::traits::Migrations;
pub struct DecibelMigrator;
impl DecibelMigrator {
    const MIGRATIONS_PATH: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/migrations");
}
impl Migrations for DecibelMigrator {
    fn get_mig_path() -> &'static str {
        return Self::MIGRATIONS_PATH;
    }
}
