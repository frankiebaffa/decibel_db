use worm_derive::WormDb;
use worm::DbContext;
#[derive(WormDb)]
pub struct Database {
    pub context: DbContext,
}

