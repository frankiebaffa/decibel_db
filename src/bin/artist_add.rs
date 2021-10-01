use {
    clap::{
        AppSettings,
        Clap
    },
    decibel_db::{
        artist::Artist,
        db::Database,
    },
    rusqlite::Error,
    worm::{
        DbCtx,
        traits::{
            primarykey::PrimaryKey,
            uniquename::UniqueName,
        },
    },
};
#[derive(Clap)]
#[clap(version = "0.0.1", author = "Frankie B")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(short, long)]
    name: String,
    #[clap(short, long)]
    bio: Option<String>,
}
fn main() -> Result<(), Error> {
    let clap: Opts = Clap::parse();
    let mut db = Database::init();
    db.context.attach_dbs();
    let artist = Artist::insert_new(&mut db, clap.name, clap.bio)?;
    println!(
        "Created new artist!\r\nid: {}\r\nname: {}\r\nbio: {}",
        artist.get_id(), artist.get_name(), artist.get_bio().unwrap_or(String::new())
    );
    Ok(())
}
