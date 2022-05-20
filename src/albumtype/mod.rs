use sqlx::FromRow;
#[derive(FromRow)]
pub struct AlbumType {
    id: i64,
    name: String,
    description: String,
    active: bool,
}
