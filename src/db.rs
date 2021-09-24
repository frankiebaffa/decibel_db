use crate::context::Context;
use rusqlite::Connection;
pub struct DbConnection {
    connection: Connection,
}
impl DbConnection {
    pub fn init(context: &Context) -> Result<DbConnection, rusqlite::Error> {
        let c = Connection::open(context.get_path_to_db())?;
        c.execute("attach 'BangersDb.db' as BangersDb", [])?;
        return Ok(DbConnection { connection: c });
    }
    pub fn get_connection(&mut self) -> &mut Connection {
        return &mut self.connection;
    }
}

