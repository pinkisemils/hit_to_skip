extern crate spoop;
extern crate clap;

use spoop::models;
use clap::{Arg, App, SubCommand};
fn get_dir() -> String {
    let app = App::new("populater")
                    .arg(Arg::with_name("scandir")
                         .short("d")
                         .long("dir")
                         .takes_value(true));
    let opts = app.get_matches();
    opts.value_of("scandir")
        .expect("No argument passed for the scannable directory")
        .to_string()
}

//fn walk_dir

fn main() {
    println!("Passed dir {}", get_dir());
}
