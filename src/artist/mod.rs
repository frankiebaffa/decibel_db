use {
    chrono::{
        DateTime,
        Local,
    },
    crate::db::traits::{
        dbmodel::DbModel,
        helpers::ColumnValue,
        primarykey::{
            PrimaryKey,
            PrimaryKeyModel,
        },
        uniquename::UniqueName,
    },
    rusqlite::{
        Connection,
        Error,
        Row,
        named_params,
    },
};
pub struct Artist {
    id: i64,
    name: String,
    bio: String,
    active: bool,
    createddate: DateTime<Local>,
    lasteditdate: DateTime<Local>,
}
impl DbModel for Artist {
    const TABLE: &'static str = "DecibelDb.Artists";
    const ALIAS: &'static str = "artist";
    const COLUMNS: &'static [str] = &[
        "Id", "Name", "Bio", "Active", "CreatedDate", "LastEditDate",
    ];
    fn from_row(row: &Row) -> Result<Self, Error> {
        let id = row.value("Id")?;
        let name = row.value("Name")?;
        let bio = row.value("Bio")?;
        let active = row.value("Active")?;
        let createddate = row.value("CreatedDate")?;
        let lasteditdate = row.value("LastEditDate")?;
        Ok(Self { id, name, bio, active, createddate, lasteditdate })
    }
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
    pub fn insert_new<'a>(c: &mut Connection, name: &'a str, bio: &'a str, active: bool) -> Result<Self, Error> {
        const INSERT_NEW_SQL: &'static str = include_str!("./sql/insert_new.sql");
        let new_id;
        {
            let mut tx = c.transaction()?;
            let sp = tx.savepoint()?;
            let mut stmt = sp.prepare(INSERT_NEW_SQL)?;
            new_id = stmt.insert(named_params!{ ":name": name, ":bio": bio, ":active": active })?;
        }
        return Self::get_by_id(c, new_id);
    }
}
