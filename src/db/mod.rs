use crate::context::Context;
use rusqlite::Connection;
pub struct Db {}
impl Db {
    pub fn attach_temp_db<'db>(connection: &'db mut Connection, context: &Context) -> Result<&'db mut Connection, rusqlite::Error> {
        let db_name = context.get_db_name();
        connection.execute(
            &format!(
                "attach ':memory:' as {}",
                db_name
            ),
            []
        )?;
        return Ok(connection);
    }
    pub fn attach_db<'db>(connection: &'db mut Connection, context: &Context) -> Result<&'db mut Connection, rusqlite::Error> {
        let db_path = context.get_db_path();
        let db_name = context.get_db_name();
        connection.execute(
            &format!(
                "attach '{}/{}.db' as {}",
                db_path,
                db_name,
                db_name
            ),
            []
        )?;
        return Ok(connection);
    }
}

