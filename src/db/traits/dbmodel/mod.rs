use rusqlite::{
    Error,
    Row
};
pub trait DbModel: Sized {
    fn from_row(row: &Row) -> Result<Self, Error>;
}
