use chrono::{DateTime, Local};
pub struct Song {
    id: i64,
    name: String,
    blurb: String,
    active: bool,
    createddate: DateTime<Local>,
    lasteditdate: DateTime<Local>,
}
impl Song {

}
