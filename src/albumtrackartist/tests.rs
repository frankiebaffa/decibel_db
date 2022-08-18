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
    let artist_id = Artist::insert(&db, "Artist One").await.unwrap();
    let artist = Artist::lookup_by_id(&db, artist_id)
        .await
        .unwrap()
        .unwrap();
    let albumtype_id = AlbumType::insert(&db, "LP").await.unwrap();
    let albumtype = AlbumType::lookup_by_id(&db, albumtype_id) 
        .await
        .unwrap()
        .unwrap();
    let album_id = Album::insert(&db, &albumtype, "Album One").await.unwrap();
    let album = Album::lookup_by_id(&db, album_id)
        .await
        .unwrap()
        .unwrap();
    let song_id = Song::insert(&db, "Song One").await.unwrap();
    let song = Song::lookup_by_id(&db, song_id)
        .await
        .unwrap()
        .unwrap();
    let albumtrack_id = AlbumTrack::insert(&db, &album, &song, 1).await.unwrap();
    let albumtrack = AlbumTrack::lookup_by_id(&db, albumtrack_id)
        .await
        .unwrap()
        .unwrap();
    let artisttype_id = ArtistType::insert(&db, "Writer", "Written By")
        .await
        .unwrap();
    let artisttype = ArtistType::lookup_by_id(&db, artisttype_id)
        .await
        .unwrap()
        .unwrap();
    let albumtrackartist_id = AlbumTrackArtist::insert(
        &db, &artist, &albumtrack, &artisttype
    ).await.unwrap();
    let albumtrackartist = AlbumTrackArtist::lookup_by_id(&db, albumtrackartist_id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(albumtrackartist.get_artist_id(), artist.get_id());
    assert_eq!(albumtrackartist.get_albumtrack_id(), albumtrack.get_id());
    assert_eq!(albumtrackartist.get_artisttype_id(), artisttype.get_id());
}
#[async_std::test]
async fn ins_albumtrackartist() {
    db_test(do_ins_albumtrackartist).await;
}
