extern crate diesel;
extern crate hyper;
extern crate rustc_serialize;

use ::models::*;
use super::*;
use ::fetch_crates::*;
use self::diesel::prelude::*;
use self::diesel::pg::PgConnection;

pub fn create_or_update_crate(crate_struct: &SubCrate, connection: &PgConnection) {
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

pub fn update_crate(crate_struct: &SubCrate, connection: &PgConnection) -> Crate {
    use ::schema::crates::dsl::*;
    let updatable = find_crate(crate_struct.to_owned(), connection).remove(0).id;
    return diesel::update(crates.find(updatable))
        .set(crate_struct)
        .get_result::<Crate>(connection)
        .expect(&format!("Unable to update"));


}
