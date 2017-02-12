#![feature(custom_derive)]
#![feature(custom_attribute)]
extern crate mpv;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_codegen;
use diesel::prelude::*;

extern crate dotenv;
extern crate taglib;
extern crate walkdir;
extern crate clap;
pub mod schema;
pub mod models;
