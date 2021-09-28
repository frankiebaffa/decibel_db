use crate::db::traits::primarykey::PrimaryKeyModel;
use rusqlite::{
    Connection,
    Error,
};
pub trait ForeignKey<U: PrimaryKeyModel>: PrimaryKeyModel {
    fn get_all_by_fk_sql() -> &'static str;
    fn get_fk_sql() -> &'static str;
    fn get_fk_name() -> &'static str;
    fn get_fk_value(&self) -> i64;
}
pub trait ForeignKeyModel<U: PrimaryKeyModel>: ForeignKey<U> {
    fn get_all_by_fk(c: &mut Connection, references: U) -> Result<Vec<Self>, Error>;
    fn get_fk(&self, c: &mut Connection) -> Result<U, Error>;
}
impl<U: PrimaryKeyModel, T: ForeignKey<U>> ForeignKeyModel<U> for T {
    fn get_all_by_fk(c: &mut Connection, references: U) -> Result<Vec<Self>, Error> {
        let mut stmt = c.prepare(Self::get_all_by_fk_sql())?;
        return stmt.query_map(&[(Self::get_fk_name(), &references.get_id())], |row| {
            Self::from_row(&row)
        })?.into_iter().collect();
    }
    fn get_fk(&self, c: &mut Connection) -> Result<U, Error> {
        let mut stmt = c.prepare(Self::get_fk_sql())?;
        return Ok(stmt.query_row(&[(Self::get_fk_name(), &self.get_fk_value())], |row| {
            U::from_row(&row)
        })?);
    }
}
