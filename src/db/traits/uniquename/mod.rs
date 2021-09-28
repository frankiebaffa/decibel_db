use rusqlite::Connection;
use crate::db::traits::dbmodel::DbModel;
pub trait UniqueName: DbModel {
    fn get_by_name_sql() -> &'static str;
}
pub trait UniqueNameModel: UniqueName {
    fn get_by_name<'n>(c: &mut Connection, name: &'n str) -> Result<Self, rusqlite::Error>;
}
impl<T: UniqueName> UniqueNameModel for T {
    fn get_by_name<'n>(c: &mut Connection, name: &'n str) -> Result<Self, rusqlite::Error> {
        let mut stmt = c.prepare(T::get_by_name_sql())?;
        return stmt.query_row(rusqlite::named_params!{ ":name": name }, |row| {
            T::from_row(&row)
        });
    }
}
