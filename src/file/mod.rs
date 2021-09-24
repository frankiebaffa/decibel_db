use chrono::{DateTime, Local};
use crate::{db::DbConnection, sql_utils::value};
use std::io::Bytes;
struct File<'f> {
    id: i64,
    fileblob: &'f [u8],
    active: bool,
    createddate: DateTime<Local>,
    lasteditdate: DateTime<Local>,
}
impl<'i> File<'i> {

}
