use worm_derive::WormDb;
use worm::DbContext;
#[derive(WormDb)]
#[db(var(name="DECIBELDBS"))]
pub struct Database {
    pub context: DbContext,
}
