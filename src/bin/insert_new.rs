use {
    clap::{
        AppSettings,
        Clap
    },
    decibel_db::{
        artist::Artist,
        album::Album,
        albumtype::AlbumType,
        artisttype::ArtistType,
        albumartist::AlbumArtist,
        db::Database,
    },
    rusqlite::Error,
    worm::{
        DbCtx,
        traits::{
            primarykey::PrimaryKey,
            uniquename::UniqueName,
            uniquename::UniqueNameModel,
        },
    },
};
#[derive(Clap)]
#[clap(version = "0.0.1", author = "Frankie B")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(subcommand)]
    which_model: WhichModel,
}
#[derive(Clap)]
enum WhichModel {
    #[clap(version = "0.0.1", author = "Frankie B")]
    #[clap(setting = AppSettings::ColoredHelp)]
    Artist(ArtistOpts),
    #[clap(version = "0.0.1", author = "Frankie B")]
    #[clap(setting = AppSettings::ColoredHelp)]
    Album(AlbumOpts),
    #[clap(version = "0.0.1", author = "Frankie B")]
    #[clap(setting = AppSettings::ColoredHelp)]
    AlbumArtist(AlbumArtistOpts),
}
#[derive(Clap)]
struct ArtistOpts {
    #[clap(long)]
    name: String,
    #[clap(long)]
    bio: Option<String>,
}
#[derive(Clap)]
struct AlbumOpts {
    #[clap(long)]
    album_type: String,
    #[clap(long)]
    name: String,
    #[clap(long)]
    blurb: Option<String>,
}
#[derive(Clap)]
struct AlbumArtistOpts {
    #[clap(long)]
    artist_name: String,
    #[clap(long)]
    album_name: String,
    #[clap(long)]
    artist_type_name: String,
}
fn main() -> Result<(), Error> {
    let clap: Opts = Clap::parse();
    let mut db = Database::init();
    db.context.attach_dbs();
    let mode = clap.which_model;
    match mode {
        WhichModel::Album(al) => insert_album(&mut db, al),
        WhichModel::Artist(ar) => insert_artist(&mut db, ar),
        WhichModel::AlbumArtist(aa) => insert_album_artist(&mut db, aa),
    }?;
    Ok(())
}
fn insert_artist(db: &mut Database, ar: ArtistOpts) -> Result<(), Error> {
    let name = ar.name;
    let bio = ar.bio;
    let artist = Artist::insert_new(db, name, bio)?;
    println!(
        "Created new artist!\r\nid: {}\r\nname: {}\r\nbio: {}",
        artist.get_id(), artist.get_name(), artist.get_bio().unwrap_or(String::new())
    );
    Ok(())
}
fn insert_album(db: &mut Database, al: AlbumOpts) -> Result<(), Error> {
    let album_type_name = al.album_type;
    let name = al.name;
    let blurb = al.blurb;
    let album_type = AlbumType::get_by_name(db, &album_type_name)?;
    let album = Album::insert_new(db, album_type.get_id(), name, blurb)?;
    println!(
        "Created new album!\r\nid: {}\r\nname: {} \r\nblurb: {}",
        album.get_id(), album.get_name(), album.get_blurb().unwrap_or(String::new())
    );
    Ok(())
}
fn insert_album_artist(db: &mut Database, aa: AlbumArtistOpts) -> Result<(), Error> {
    use std::io::BufRead;
    let artist_name = aa.artist_name;
    let artist = Artist::get_by_name(db, &artist_name)?;
    let album_name = aa.album_name;
    let albums = Album::get_all_by_name(db, album_name)?;
    let album;
    if albums.len() == 1 {
        album = albums.get(0).unwrap();
    } else {
        for i in 0..albums.len()-1 {
            let a = albums.get(i).unwrap();
            println!("{}: {}, {}", i, a.get_name(), a.get_blurb().unwrap_or(String::new()));
        }
        let stdin = std::io::stdin();
        let mut lock = stdin.lock();
        let mut input = String::new();
        lock.read_line(&mut input).unwrap();
        let num = input.trim().parse::<usize>().unwrap();
        album = albums.get(num).unwrap();
    }
    let artist_type_name = aa.artist_type_name;
    let artist_type = ArtistType::get_by_name(db, &artist_type_name)?;
    let aartist = AlbumArtist::insert_new(db, artist.get_id(), album.get_id(), artist_type.get_id())?;
    println!(
        "Created new album artist!\r\nid: {}\r\nalbum name: {}\r\nartist name: {}\r\nartist type: {}",
        aartist.get_id(), album.get_name(), artist.get_name(), artist_type.get_name()
    );
    Ok(())
}
