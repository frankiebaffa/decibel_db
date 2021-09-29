use {
    chrono::{DateTime, Local},
    worm::traits::{
        dbmodel::DbModel,
        primarykey::PrimaryKey,
        uniquename::UniqueName,
    },
    worm_derive::Worm,
};
#[derive(Worm)]
#[dbmodel(table(db="DecibelDb",name="ArtistTypes",alias="artisttypes"))]
pub struct ArtistType {
    #[dbcolumn(column(name="Id"))]
    id: i64,
    #[dbcolumn(column(name="Name"))]
    name: String,
    #[dbcolumn(column(name="Descriptor"))]
    descriptor: String,
    #[dbcolumn(column(name="Description"))]
    description: String,
    #[dbcolumn(column(name="Active"))]
    active: bool,
    #[dbcolumn(column(name="CreatedDate"))]
    createddate: DateTime<Local>,
    #[dbcolumn(column(name="LastEditDate"))]
    lasteditdate: DateTime<Local>,
}
impl PrimaryKey for ArtistType {
    const PRIMARY_KEY: &'static str = "Id";
    fn get_id(&self) -> i64 {
        return self.id;
    }
}
impl UniqueName for ArtistType {
    const NAME: &'static str = "Name";
    fn get_name(&self) -> String {
        return self.name.clone();
    }
}
impl ArtistType {
    pub const PERFORMER: &'static str = "Performer";
    pub const WRITER: &'static str = "Writer";
    pub const COMPOSER: &'static str = "Composer";
    pub const PRODUCER: &'static str = "Producer";
    pub const FEATURE: &'static str = "Feature";
}
