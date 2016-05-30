#![deny(warnings)]
#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(diesel_codegen)]

extern crate diesel;
extern crate fresh_cargo;
extern crate hyper;
extern crate rustc_serialize;

use self::fresh_cargo::models::*;
use self::fresh_cargo::fetch_crates::*;
use self::diesel::prelude::*;
use diesel::pg::PgConnection;
use self::fresh_cargo::*;

fn main() {
    let new_crates = new_crates();
    let updated_crates = updated_crates();
    let connection = establish_connection();

    for crate_struct in new_crates.iter() {
        create_or_update_crate(crate_struct.to_owned(), &connection);
    }

    for crate_struct in updated_crates.iter() {
        create_or_update_crate(crate_struct.to_owned(), &connection);
    }

}

fn create_or_update_crate(crate_struct: &SubCrate, connection: &PgConnection) {
    if crate_exists(crate_struct.to_owned(), connection) {
        println!("Updating: {}", crate_struct.name);
        println!("        : {}", crate_struct.url);
        println!("        : {}", crate_struct.description);
        println!("        : {}", crate_struct.version);
        update_crate(crate_struct.to_owned(), connection);
    } else {
        println!("Creating: {}", crate_struct.name);
        println!("        : {}", crate_struct.url);
        println!("        : {}", crate_struct.description);
        println!("        : {}", crate_struct.version);
        create_crate(connection,
                     &*crate_struct.name,
                     &*crate_struct.url,
                     &*crate_struct.description,
                     &*crate_struct.version);
    }
}

fn update_crate(crate_struct: &SubCrate, connection: &PgConnection) -> Crate {
    use fresh_cargo::schema::crates::dsl::*;
    let updatable = find_crate(crate_struct.to_owned(), connection).remove(0).id;
    return diesel::update(crates.find(updatable))
        .set(crate_struct)
        .get_result::<Crate>(connection)
        .expect(&format!("Unable to update"));


}
