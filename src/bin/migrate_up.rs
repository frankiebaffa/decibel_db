use decibel_db::DecibelMigrator;
use worm_derive::WormDb;
use worm::{DbCtx, DbContext};
use migaton::traits::DoMigrations;
#[derive(WormDb)]
#[db(var(name="DECIBELDBS"))]
struct Database {
    context: DbContext,
}
fn main() {
    let mut mem_db = Database::init();
    mem_db.context.attach_temp_dbs();
    let mut db = Database::init();
    db.context.attach_dbs();
    let skips = DecibelMigrator::migrate_up(&mut mem_db, &mut db);
    println!("{} migrations were skipped", skips);
}
