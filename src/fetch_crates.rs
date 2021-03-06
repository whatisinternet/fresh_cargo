extern crate diesel;
extern crate hyper;
extern crate rustc_serialize;

use ::models::*;
use super::*;
use self::diesel::prelude::*;
use diesel::pg::PgConnection;
use std::io::Read;
use self::hyper::Client;
use self::hyper::header::ContentType;
use self::rustc_serialize::json::Json;
use ::schema::crates;

#[changeset_for(crates)]
pub struct SubCrate {
    pub name: String,
    pub url: String,
    pub description: String,
    pub version: String,
    pub published: bool,
}

impl SubCrate {
    pub fn publish_setting(&mut self, publish: bool) {
        self.published = publish;
    }
}

pub fn find_crate(crate_struct: &SubCrate, connection: &PgConnection) -> Vec<Crate> {
    use ::schema::crates::dsl::*;

    return crates.filter(name.eq(crate_struct.name.to_owned()))
        .limit(1)
        .load::<Crate>(connection)
        .expect("Error loading crates");
}

pub fn crate_exists(crate_struct: &SubCrate, connection: &PgConnection) -> bool {
    let results = find_crate(crate_struct, connection);
    if results.len() > 0 {
        return true;
    } else {
        return false;
    }
}


pub fn updated_crates() -> Vec<SubCrate> {
    let client = Client::new();
    let connection = establish_connection();

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
            let mut temp_crate = SubCrate {
                name: get_string_key(crate_json.to_owned(), "name"),
                url: get_url(crate_json.to_owned()),
                description: get_string_key(crate_json.to_owned(), "description"),
                version: get_string_key(crate_json.to_owned(), "max_version"),
                published: false,
            };
            if crate_exists(&temp_crate, &connection) {
                let crate_version = find_crate(&temp_crate, &connection).remove(0).version;
                if temp_crate.version != crate_version {
                    temp_crate.published = false;
                } else {
                    temp_crate.publish_setting(true);
                }
            }
            return temp_crate;
        })
        .collect();
}

pub fn new_crates() -> Vec<SubCrate> {
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
                published: false,
            }
        })
        .collect();
}

fn get_new_crates(json: Json) -> Vec<Json> {
    return json.find_path(&["new_crates"])
        .unwrap_or(&Json::from_str("[]").unwrap())
        .as_array()
        .unwrap()
        .to_owned();
}

fn get_just_updated(json: Json) -> Vec<Json> {
    return json.find_path(&["just_updated"])
        .unwrap_or(&Json::from_str("[]").unwrap())
        .as_array()
        .unwrap()
        .to_owned();
}

fn get_url(json: Json) -> String {
    return format!("https://crates.io/crates/{}", get_string_key(json, "name"));
}

fn get_string_key(json: Json, key: &str) -> String {
    return json.find_path(&[key])
        .unwrap_or(&Json::from_str("\"nothing\"").unwrap())
        .to_string()
        .replace("\"", "");
}
