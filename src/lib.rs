pub mod album;
pub mod albumartist;
pub mod albumtrackartist;
pub mod albumtrack;
pub mod albumtype;
pub mod artist;
pub mod artisttype;
pub mod constants;
pub mod context;
pub mod file;
pub mod song;
pub struct Migrations;
use {
    migaton::Migrator,
    worm::DbCtx,
};
impl Migrations {
    const MIGRATIONS_PATH: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/migrations");
    pub fn migrate_up(mem_db: &mut impl DbCtx, db: &mut impl DbCtx) -> usize {
        let mut mem_c = mem_db.use_connection();
        let mut c = db.use_connection();
        let skips = match Migrator::do_up(&mut mem_c, &mut c, Self::MIGRATIONS_PATH) {
            Ok(res) => res,
            Err(e) => panic!("{}", e),
        };
        return skips;
    }
    pub fn migrate_down(mem_db: &mut impl DbCtx, db: &mut impl DbCtx) -> usize {
        let mut mem_c = mem_db.use_connection();
        let mut c = db.use_connection();
        let skips = match Migrator::do_down(&mut mem_c, &mut c, Self::MIGRATIONS_PATH) {
            Ok(res) => res,
            Err(e) => panic!("{}", e),
        };
        return skips;
    }
}
