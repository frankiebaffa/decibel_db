use migaton::Migrator;
use decibel_db::db::Database;
use worm::DbCtx;
fn main() {
    let mut mem_db = Database::init();
    mem_db.context.attach_temp_dbs();
    let mut db = Database::init();
    db.context.attach_dbs();
    let mut mem_c = mem_db.context.use_connection();
    let mut c = db.context.use_connection();
    let skips = match Migrator::do_down(&mut mem_c, &mut c, "./migrations") {
        Ok(res) => res,
        Err(e) => panic!("{}", e),
    };
    println!("{} migrations were skipped", skips);
}
