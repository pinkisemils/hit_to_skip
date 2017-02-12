use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use std::time::SystemTime;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[derive(Queryable)]
pub struct Track {
    pub id: i32,
    pub path: String,
    pub title: String,
    pub album: String,
    pub artist: String,
}

#[derive(Insertable)]
#[table_name="tracks"]
pub struct InsTrack<'a> {
    pub path: &'a str,
    pub title: &'a str,
    pub album: &'a str,
    pub artist: &'a str,
}

#[derive(Queryable)]
pub struct User {
    pub user_id: String,
    pub allowance: i32,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct InsUser<'a> {
    pub path: &'a str,
    pub allowance: i32,
}

#[derive(Queryable)]
pub struct Rank {
    pub song_id: i32,
    pub user_id: String,
    pub timestamp: SystemTime,
}

#[derive(Insertable)]
#[table_name="ranks"]
pub struct InsRank<'a> {
    pub path: &'a str,
    pub allowance: i32,
    pub timestamp: &'a SystemTime,
}
