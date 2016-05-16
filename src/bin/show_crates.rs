extern crate fresh_cargo;
extern crate diesel;

use self::fresh_cargo::*;
use self::fresh_cargo::models::*;
use self::diesel::prelude::*;

fn main() {
    use fresh_cargo::schema::crates::dsl::*;

    let connection = establish_connection();
    let results = crates.load::<Crate>(&connection)
        .expect("Error loading crates");

    println!("Displaying {} crates", results.len());
    for _crate in results {
        println!("{0: <50} | {1: <20} | {2: <20} | {3: <20}",
                 _crate.name,
                 _crate.version,
                 _crate.published,
                 _crate.description);
    }
}
