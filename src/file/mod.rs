use chrono::{DateTime, Local};
struct File<'f> {
    id: i64,
    fileblob: &'f [u8],
    active: bool,
    createddate: DateTime<Local>,
    lasteditdate: DateTime<Local>,
}
impl<'i> File<'i> {

}
