#![deny(warnings)]
#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(diesel_codegen)]
#![plugin(diesel_codegen, dotenv_macros)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate rustc_serialize;

pub mod schema;
pub mod models;
pub mod fetch_crates;
pub mod create_update;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use self::models::{Crate, NewCrate};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn create_crate<'a>(conn: &PgConnection,
                        name: &'a str,
                        url: &'a str,
                        description: &'a str,
                        version: &'a str)
                        -> Crate {
    use schema::crates;

    let new_crate = NewCrate {
        name: name,
        url: url,
        description: description,
        version: version,
        published: false,
    };

    diesel::insert(&new_crate)
        .into(crates::table)
        .get_result(conn)
        .expect("Error saving new crate")
}
