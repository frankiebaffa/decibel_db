use {
    crate::{
        album::Album,
        albumtype::AlbumType,
        albumtrack::AlbumTrack,
        albumtrackartist::AlbumTrackArtist,
        artist::Artist,
        artisttype::ArtistType,
        migrator::DecibelMigrator,
        song::Song,
        test_utils::db_test,
    },
    sqlx::SqlitePool,
};
async fn do_ins_albumtrackartist(db: SqlitePool) {
    DecibelMigrator::migrate(&db).await.unwrap();
    let artist = Artist::insert(&db, "Artist One").await.unwrap();
    let albumtype = AlbumType::always(&db, "LP").await.unwrap();
    let album = Album::insert(&db, &albumtype, "Album One").await.unwrap();
    let song = Song::insert(&db, "Song One").await.unwrap();
    let albumtrack = AlbumTrack::insert(&db, &album, &song, 1).await.unwrap();
    let artisttype = ArtistType::always(&db, "Writer", "Written By").await.unwrap();
    let albumtrackartist = AlbumTrackArtist::insert(
        &db, &artist, &albumtrack, &artisttype
    ).await.unwrap();
    assert_eq!(albumtrackartist.get_artist_id(), artist.get_id());
    assert_eq!(albumtrackartist.get_albumtrack_id(), albumtrack.get_id());
    assert_eq!(albumtrackartist.get_artisttype_id(), artisttype.get_id());
}
#[async_std::test]
async fn ins_albumtrackartist() {
    db_test(do_ins_albumtrackartist).await;
}
