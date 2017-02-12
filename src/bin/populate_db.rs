extern crate spoop;
extern crate clap;
extern crate diesel;
extern crate walkdir;
extern crate id3;

use id3::Tag;
use spoop::models;
use spoop::schema;
use clap::{Arg, App};
use self::diesel::prelude::*;
use self::diesel::pg::PgConnection;
use walkdir::WalkDir;

fn get_dir() -> String {
    let app = App::new("populater").arg(Arg::with_name("scandir")
        .short("d")
        .long("dir")
        .takes_value(true));
    let opts = app.get_matches();
    opts.value_of("scandir")
        .expect("No argument passed for the scannable directory")
        .to_string()
}

fn walk_dir(root_path: String, conn: &PgConnection) {
    use schema::tracks;
    let mut count = 0;
    for path in WalkDir::new(root_path.clone()).into_iter() {
        if let Ok(p) = path {
            if let Ok(tags) = Tag::read_from_path(p.path()
                .to_str()
                .expect("Couldn't even convert path to string")) {
                let all_tags = [tags.title().unwrap_or(""),
                                tags.album().unwrap_or(""),
                                tags.artist().unwrap_or("")];
                if all_tags.iter()
                    .any(|t| t.len() == 0) {
                    println!("No tags");
                    continue;
                }
                let new_track = models::InsTrack {
                    path: &("".to_string() + &root_path +
                            p.path().to_str().expect("Couldn't even convert path to string")),
                    title: &all_tags[0],
                    album: &all_tags[1],
                    artist: &all_tags[2],
                };
                match diesel::insert(&new_track)
                    .into(tracks::table)
                    .get_result::<models::Track>(conn) {
                    Ok(_) => count += 1,
                    Err(e) => {
                        println!("Encountered error whilst inserting {:?} - {:?}", p, e);
                    }
                }
            } else {
                println!("Failed to get tag for {:?}", p);
                continue;
            }
        } else {
            println!("Failed to open file {:?}", path);
            continue;
        }
    }
    println!("Inserted {} records", count);
}

fn main() {
    println!("Passed dir {}", get_dir());
    walk_dir(get_dir(), &models::establish_connection());
}
