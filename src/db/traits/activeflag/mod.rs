use crate::db::traits::dbmodel::DbModel;
use rusqlite::Connection;
pub trait ActiveFlag: DbModel {
    fn get_all_active_sql() -> &'static str;
}
pub trait ActiveFlagModel: ActiveFlag {
    fn get_all_active(c: &mut Connection) -> Result<Vec<Self>, rusqlite::Error>;
}
impl<T: ActiveFlag> ActiveFlagModel for T {
    fn get_all_active(c: &mut Connection) -> Result<Vec<T>, rusqlite::Error> {
        let mut stmt = c.prepare(T::get_all_active_sql())?;
        return stmt.query_map([], |row| {
            T::from_row(&row)
        })?.into_iter().collect();
    }
}
