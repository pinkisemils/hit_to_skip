use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::types::Timestamp;
use diesel;
use dotenv::dotenv;
use std::env;
use std::time::SystemTime;
use diesel::types::ToSql;
use std::hash::Hash;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[derive(Queryable, Associations,HasTable)]
#[table_name="tracks"]
#[has_many(ranks)]
pub struct Track {
    pub track_id: i32,
    pub path: String,
    pub title: String,
    pub album: String,
    pub artist: String,
}

impl diesel::associations::Identifiable for Track {
    type Id=u64;
    fn id(self) -> Self::Id {
        self.track_id.hash()
    }
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

#[derive(Queryable, Associations,HasTable)]
#[table_name="ranks"]
pub struct Rank {
    pub track_id: i32,
    pub user_id: String,
    pub timestamp: SystemTime,
}

impl diesel::associations::Identifiable for Rank {
    type Id=u64;
    fn id(self) -> Self::Id {
        (self.track_id, self.user_id).hash()
    }
}

use super::schema::ranks;
#[derive(Insertable)]
#[table_name="ranks"]
pub struct InsRank<'a> {
    pub track_id: i32,
    pub user_id: &'a str,
}
