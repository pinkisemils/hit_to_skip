use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::types::Timestamp;
use diesel;
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

#[primary_key(track_id)]
#[derive(Queryable, Associations, Identifiable)]
#[table_name="tracks"]
#[has_many(ranks)]
pub struct Track {
    pub track_id: i32,
    pub path: String,
    pub title: String,
    pub album: String,
    pub artist: String,
}

use super::schema::tracks;
#[derive(Insertable)]
#[has_many(ranks)]
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

use super::schema::users;
#[derive(Insertable)]
#[table_name="users"]
pub struct InsUser<'a> {
    pub user_id: &'a str,
    pub allowance: i32,
}

#[primary_key(track_id,user_id)]
#[derive(Queryable, Associations)]
#[table_name="ranks"]
pub struct Rank {
    pub track_id: i32,
    pub user_id: String,
    pub timestamp: SystemTime,
}

use super::schema::ranks;
#[derive(Insertable)]
#[table_name="ranks"]
pub struct InsRank<'a> {
    pub track_id: i32,
    pub user_id: &'a str,
}
