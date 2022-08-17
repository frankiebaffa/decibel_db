use {
    crate::{
        album::Album,
        albumartist::AlbumArtist,
        albumtype::AlbumType,
        artist::Artist,
        artisttype::ArtistType,
        migrator::DecibelMigrator,
        test_utils::db_test,
    },
    sqlx::SqlitePool,
};
async fn do_ins_albumartist(db: SqlitePool) {
    DecibelMigrator::migrate(&db).await.unwrap();
    let artist_id = Artist::insert(&db, "Artist One").await.unwrap();
    let artist = Artist::lookup_by_id(&db, artist_id)
        .await
        .unwrap();
    let albumtype = AlbumType::always(&db, "LP")
        .await
        .unwrap();
    let album_id = Album::insert(&db, &albumtype, "Album One").await.unwrap();
    let album = Album::lookup_by_id(&db, album_id)
        .await
        .unwrap();
    let artisttype = ArtistType::always(&db, "Writer", "Written By")
        .await
        .unwrap();
    let albumartist_id = AlbumArtist::insert(&db, &artist, &album, &artisttype)
        .await
        .unwrap();
    let albumartist = AlbumArtist::lookup_by_id(&db, albumartist_id)
        .await
        .unwrap();
    assert_eq!(albumartist.get_album_id(), album.get_id());
    assert_eq!(albumartist.get_artist_id(), artist.get_id());
    assert_eq!(albumartist.get_artisttype_id(), artisttype.get_id());
}
#[async_std::test]
async fn ins_albumartist() {
    db_test(do_ins_albumartist).await;
}
