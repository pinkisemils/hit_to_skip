use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::types::Timestamp;
use dotenv::dotenv;
use std::env;
use std::time::SystemTime;
use diesel::types::ToSql;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[derive(Queryable)]
#[has_many(ranks)]
pub struct Track {
    pub id: i32,
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

#[derive(Queryable)]
pub struct Rank {
    pub song_id: i32,
    pub user_id: String,
    pub timestamp: SystemTime,
}

use super::schema::ranks;
#[derive(Insertable)]
#[table_name="ranks"]
pub struct InsRank<'a> {
    pub song_id: i32,
    pub user_id: &'a str,
}
