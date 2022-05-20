use sqlx::FromRow;
#[derive(FromRow)]
pub struct ArtistType {
    id: i64,
    name: String,
    descriptor: String,
    description: Option<String>,
    active: bool,
}
