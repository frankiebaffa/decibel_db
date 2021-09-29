use {
    chrono::{
        DateTime,
        Local,
    },
    worm::traits::{
        activeflag::ActiveFlag,
        dbmodel::DbModel,
        primarykey::PrimaryKey,
    },
    worm_derive::Worm,
};
#[derive(Worm)]
#[dbmodel(table(db="DecibelDb",name="Songs",alias="song"))]
pub struct Song {
    #[dbcolumn(column(name="Id"))]
    id: i64,
    #[dbcolumn(column(name="Name"))]
    name: String,
    #[dbcolumn(column(name="Blurb"))]
    blurb: String,
    #[dbcolumn(column(name="Active"))]
    active: bool,
    #[dbcolumn(column(name="CreatedDate"))]
    createddate: DateTime<Local>,
    #[dbcolumn(column(name="LastEditDate"))]
    lasteditdate: DateTime<Local>,
}
impl PrimaryKey for Song {
    const PRIMARY_KEY: &'static str = "Id";
    fn get_id(&self) -> i64 {
        return self.id;
    }
}
impl ActiveFlag for Song {
    const ACTIVE: &'static str = "Active";
    fn get_active(&self) -> bool {
        return self.active;
    }
}
