use rusqlite::{types::FromSql, Row};
pub fn value<'a, T>(row: &Row, column: &'a str) -> Result<T, rusqlite::Error> where T: FromSql{
    let column_index = row.column_index(column)?;
    let column_val = row.get(column_index)?;
    return Ok(column_val);
}

