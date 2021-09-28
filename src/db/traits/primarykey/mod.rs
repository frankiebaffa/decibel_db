use crate::db::traits::dbmodel::DbModel;
use rusqlite::Connection;
pub trait PrimaryKey: DbModel {
    fn get_by_id_sql() -> &'static str;
    fn get_id(&self) -> i64;
}
pub trait PrimaryKeyModel: PrimaryKey {
    fn get_by_id(c: &mut Connection, id: i64) -> Result<Self, rusqlite::Error>;
}
impl<T: PrimaryKey> PrimaryKeyModel for T {
    fn get_by_id(c: &mut Connection, id: i64) -> Result<Self, rusqlite::Error> {
        let mut stmt = c.prepare(Self::get_by_id_sql())?;
        return stmt.query_row(&[(":id", &id)], |row| {
            Self::from_row(&row)
        });
    }
}
