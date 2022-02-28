use {
    chrono::{
        DateTime,
        Utc,
    },
    crate::error::{ DecibelDbErr, DecibelDbError, },
    worm::{
        derive::Worm,
        core::{ DbCtx, UniqueNameModel, },
    },
};
#[derive(Worm)]
#[dbmodel(table(schema="DecibelDb",name="Artists",alias="artist"))]
pub struct Artist {
    #[dbcolumn(column(name="Id", primary_key))]
    id: i64,
    #[dbcolumn(column(name="Name", unique_name, insertable))]
    name: String,
    #[dbcolumn(column(name="Bio", insertable))]
    bio: Option<String>,
    #[dbcolumn(column(name="Active", active_flag))]
    active: bool,
    #[dbcolumn(column(name="CreatedDate", insertable, utc_now))]
    createddate: DateTime<Utc>,
    #[dbcolumn(column(name="LastEditDate", insertable, utc_now))]
    lasteditdate: DateTime<Utc>,
}
impl Artist {
    pub fn get_existing_or_insert_new(
        db: &mut impl DbCtx, name: impl AsRef<str>
    ) -> Result<Self, DecibelDbErr> {
        match Artist::get_by_name(db, name.as_ref()).quick() {
            Ok(artist) => return Ok(artist),
            Err(_) => {},
        }
        Artist::insert_new(db, name.as_ref().to_string(), None).quick()
    }
}
