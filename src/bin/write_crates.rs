#![deny(warnings)]
#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(diesel_codegen)]

extern crate fresh_cargo;

use self::fresh_cargo::*;
use self::fresh_cargo::create_update::*;
use self::fresh_cargo::fetch_crates::*;
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
