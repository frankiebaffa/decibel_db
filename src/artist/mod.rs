use {
    chrono::{
        DateTime,
        Local,
    },
    worm::traits::{
        dbmodel::DbModel,
        primarykey::PrimaryKey,
        uniquename::UniqueName,
    },
    worm_derive::Worm,
};
#[derive(Worm)]
#[dbmodel(table(db="DecibelDb", name="Artists", alias="artist", bool_flag="Active"))]
pub struct Artist {
    #[dbcolumn(column(name="Id"))]
    id: i64,
    #[dbcolumn(column(name="Name"))]
    name: String,
    #[dbcolumn(column(name="Bio"))]
    bio: String,
    #[dbcolumn(column(name="Active"))]
    active: bool,
    #[dbcolumn(column(name="CreatedDate"))]
    createddate: DateTime<Local>,
    #[dbcolumn(column(name="LastEditDate"))]
    lasteditdate: DateTime<Local>,
}
impl PrimaryKey for Artist {
    const PRIMARY_KEY: &'static str = "Id";
    fn get_id(&self) -> i64 {
        return self.id;
    }
}
impl UniqueName for Artist {
    const NAME: &'static str = "Name";
    fn get_name(&self) -> String {
        return self.name.clone();
    }
}
impl Artist {
    pub fn row<'a>(row: &rusqlite::Row) -> Result<Artist, rusqlite::Error> {
        return Artist::from_row(row);
    }
    //pub fn insert_new<'a>(c: &mut Connection, name: &'a str, bio: &'a str, active: bool) -> Result<Self, Error> {
    //    const INSERT_NEW_SQL: &'static str = include_str!("./sql/insert_new.sql");
    //    let new_id;
    //    {
    //        let mut tx = c.transaction()?;
    //        let sp = tx.savepoint()?;
    //        let mut stmt = sp.prepare(INSERT_NEW_SQL)?;
    //        new_id = stmt.insert(named_params!{ ":name": name, ":bio": bio, ":active": active })?;
    //    }
    //    return Self::get_by_id(c, new_id);
    //}
}
