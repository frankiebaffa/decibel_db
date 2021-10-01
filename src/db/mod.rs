use worm_derive::WormDb;
use worm::DbContext;
#[derive(WormDb)]
#[db(var(name="WORMDBS"))]
pub struct Database {
    pub context: DbContext,
}

