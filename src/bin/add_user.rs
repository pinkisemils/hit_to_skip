#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate diesel;
extern crate rocket;
extern crate rocket_codegen;

extern crate serde_json;
#[macro_use]
extern crate serde_derive;


#[macro_use]
extern crate rocket_contrib;

extern crate spoop;
use rocket::response::content;
use rocket::http;
use rocket_contrib::{JSON, Value};

use spoop::models;
use spoop::schema;
use self::diesel::prelude::*;
use self::diesel::pg::PgConnection;


#[derive(Serialize)]
struct Results {
    candidates: Vec<models::Track>,
}



#[get("/search/<term>",)]
fn search(term: &str) -> JSON<Results> {
    let conn = models::establish_connection();
    use schema::tracks::dsl::*;
    let query_str = "%".to_string();
    if let Ok(matching_tracks) = tracks.filter(title.like(query_str + term + "%"))
                                .limit(100)
                                .load::<models::Track>(&conn){
                                    JSON(Results{candidates:matching_tracks})
                                } else {
                                    println!("Fail");
                                    JSON(Results{candidates: vec![]})
                                }
}


fn main() {
    println!("This should handle the web api");
    rocket::ignite().mount("/", routes![search]).launch();
}


