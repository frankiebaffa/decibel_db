use worm::derive::Worm;
#[derive(Worm)]
#[dbmodel(table(schema="DecibelDb",name="ArtistTypes",alias="artisttypes"))]
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
}
