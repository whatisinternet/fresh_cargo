extern crate fresh_cargo;
extern crate diesel;
extern crate hyper;
extern crate rustc_serialize;

use self::fresh_cargo::models::*;
use self::diesel::prelude::*;
use self::fresh_cargo::*;
use std::io::Read;
use self::hyper::Client;
use self::hyper::header::ContentType;
use self::rustc_serialize::json::Json;

struct SubCrate {
    pub name: String,
    pub url: String,
    pub description: String,
    pub version: String,
}

fn main() {
    let new_crates = new_crates();
    let updated_crates = updated_crates();

    for crate_struct in new_crates.iter() {
        create_or_update_crate(crate_struct.to_owned());
    }

    for crate_struct in updated_crates.iter() {
        create_or_update_crate(crate_struct.to_owned());
    }

}

fn create_or_update_crate(crate_struct: &SubCrate) -> Crate {
    if crate_exists(crate_struct.to_owned()) {
        println!("Updating: {}", crate_struct.name);
        return update_crate(crate_struct.to_owned());
    } else {
        println!("Creating: {}", crate_struct.name);
        let connection = establish_connection();
        return create_crate(&connection,
                            &*crate_struct.name,
                            &*crate_struct.url,
                            &*crate_struct.description,
                            &*crate_struct.version);
    }
}

fn update_crate(crate_struct: &SubCrate) -> Crate {
    use fresh_cargo::schema::crates::dsl::{crates, published, version};
    let connection = establish_connection();
    let connection_two = establish_connection();

    let updatable = find_crate(crate_struct.to_owned()).remove(0).id;
    let crate_version = find_crate(crate_struct.to_owned()).remove(0).version.to_owned();
    let mut should_publish = false;
    if crate_struct.version != crate_version {
        should_publish = true;
    }

    let updated = diesel::update(crates.find(updatable))
        .set(version.eq(&*crate_struct.version))
        .get_result::<Crate>(&connection)
        .expect(&format!("Unable to find crate"));

    return diesel::update(crates.find(updated.id))
        .set(published.eq(&should_publish))
        .get_result::<Crate>(&connection_two)
        .expect(&format!("Unable to find crate"));
}

fn find_crate(crate_struct: &SubCrate) -> Vec<Crate> {
    use fresh_cargo::schema::crates::dsl::*;
    let connection = establish_connection();

    return crates.filter(name.eq(crate_struct.name.to_owned()))
        .limit(1)
        .load::<Crate>(&connection)
        .expect("Error loading crates");
}

fn crate_exists(crate_struct: &SubCrate) -> bool {
    let results = find_crate(crate_struct);
    if results.len() > 0 {
        return true;
    } else {
        return false;
    }
}

fn updated_crates() -> Vec<SubCrate> {
    let client = Client::new();

    let mut result = client.get("https://crates.io/summary")
        .header(ContentType::json())
        .send()
        .unwrap();

    let mut body = String::new();
    result.read_to_string(&mut body)
        .unwrap();

    let json = Json::from_str(&body).unwrap();
    let new_crates = get_just_updated(json.to_owned());

    return new_crates.iter()
        .map(|crate_json| {
            SubCrate {
                name: get_string_key(crate_json.to_owned(), "name"),
                url: get_url(crate_json.to_owned()),
                description: get_string_key(crate_json.to_owned(), "description"),
                version: get_string_key(crate_json.to_owned(), "max_version"),
            }
        })
        .collect();
}

fn new_crates() -> Vec<SubCrate> {
    let client = Client::new();

    let mut result = client.get("https://crates.io/summary")
        .header(ContentType::json())
        .send()
        .unwrap();

    let mut body = String::new();
    result.read_to_string(&mut body)
        .unwrap();

    let json = Json::from_str(&body).unwrap();
    let new_crates = get_new_crates(json.to_owned());

    return new_crates.iter()
        .map(|crate_json| {
            SubCrate {
                name: get_string_key(crate_json.to_owned(), "name"),
                url: get_url(crate_json.to_owned()),
                description: get_string_key(crate_json.to_owned(), "description"),
                version: get_string_key(crate_json.to_owned(), "max_version"),
            }
        })
        .collect();
}

fn get_new_crates(json: Json) -> Vec<Json> {
    return json.find_path(&["new_crates"])
        .unwrap()
        .as_array()
        .unwrap()
        .to_owned();
}

fn get_just_updated(json: Json) -> Vec<Json> {
    return json.find_path(&["just_updated"])
        .unwrap()
        .as_array()
        .unwrap()
        .to_owned();
}

fn get_url(json: Json) -> String {
    return format!("https://crates.io/crates/{}", get_string_key(json, "name"));
}

fn get_string_key(json: Json, key: &str) -> String {
    return json.find_path(&[key])
        .unwrap()
        .to_string()
        .replace("\"", "");
}
