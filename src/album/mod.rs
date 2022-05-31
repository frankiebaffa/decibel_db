use {
    chrono::{
        DateTime,
        Utc,
    },
    sqlx::FromRow
};
#[derive(FromRow)]
pub struct Album {
    id: i64,
    albumtype_id: i64,
    cover_id: i64,
    name: String,
    blurb: Option<String>,
    active: bool,
    created_date: DateTime<Utc>,
    last_edit_date: DateTime<Utc>,
}
impl Album {
    pub fn get_id(&self) -> i64 {
        self.id
    }
    pub fn get_albumtype_id(&self) -> i64 {
        self.albumtype_id
    }
    pub fn get_cover_id(&self) -> i64 {
        self.cover_id
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_blurb(&self) -> Option<String> {
        self.blurb.clone()
    }
    pub fn get_active(&self) -> bool {
        self.active
    }
    pub fn get_created_date(&self) -> DateTime<Utc> {
        self.created_date.clone()
    }
    pub fn get_last_edit_date(&self) -> DateTime<Utc> {
        self.last_edit_date.clone()
    }
}
