use {
    chrono::{DateTime, Local},
    crate::db::{
        Database,
        AttachedToDatabase,
    },
    worm_derive::Worm,
};
#[derive(Worm)]
#[dbmodel(table(db="Database",schema="DecibelDb",name="ArtistTypes",alias="artisttypes"))]
pub struct ArtistType {
    #[dbcolumn(column(name="Id", primary_key))]
    id: i64,
    #[dbcolumn(column(name="Name", unique_name))]
    name: String,
    #[dbcolumn(column(name="Descriptor"))]
    descriptor: String,
    #[dbcolumn(column(name="Description"))]
    description: Option<String>,
    #[dbcolumn(column(name="Active", active_flag))]
    active: bool,
    #[dbcolumn(column(name="CreatedDate"))]
    createddate: DateTime<Local>,
    #[dbcolumn(column(name="LastEditDate"))]
    lasteditdate: DateTime<Local>,
}
impl ArtistType {
    pub const PERFORMER: &'static str = "Performer";
    pub const WRITER: &'static str = "Writer";
    pub const COMPOSER: &'static str = "Composer";
    pub const PRODUCER: &'static str = "Producer";
    pub const FEATURE: &'static str = "Feature";
}
