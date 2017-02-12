#![feature(plugin)]
#![plugin(rocket_codegen)]

#![feature(custom_derive)]
#![feature(custom_attribute)]
extern crate mpv;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_codegen;
use diesel::prelude::*;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate dotenv;
extern crate taglib;
extern crate walkdir;
extern crate clap;
pub mod schema;
pub mod models;
