use {
    chrono::{
        DateTime,
        Local,
    },
    crate::albumtype::AlbumType,
    worm::traits::{
        activeflag::ActiveFlag,
        dbmodel::DbModel,
        foreignkey::ForeignKey,
        primarykey::PrimaryKey,
    },
    worm_derive::Worm,
};
#[derive(Worm)]
#[dbmodel(table(db="DecibelDb",name="Albums",alias="Album",bool_flag="Active"))]
pub struct Album {
    #[dbcolumn(column(name="Id"))]
    id: i64,
    #[dbcolumn(column(name="AlbumType_Id"))]
    albumtype_id: i64,
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
impl PrimaryKey for Album {
    const PRIMARY_KEY: &'static str = "Id";
    fn get_id(&self) -> i64 {
        return self.id;
    }
}
impl ActiveFlag for Album {
    const ACTIVE: &'static str = "Active";
    fn get_active(&self) -> bool {
        return self.active;
    }
}
impl ForeignKey<AlbumType> for Album {
    const FOREIGN_KEY: &'static str = "AlbumType_Id";
    const FOREIGN_KEY_PARAM: &'static str = ":albumtype_id";
    fn get_fk_value(&self) -> i64 {
        return self.albumtype_id;
    }
}
