extern crate iron;
extern crate router;
extern crate handlebars_iron as hbs;
extern crate dotenv;
extern crate rustc_serialize;
extern crate fresh_cargo;
extern crate diesel;

use self::iron::prelude::*;
use self::iron::{status};
use self::router::Router;
use self::hbs::{Template, HandlebarsEngine, DirectorySource, MemorySource};
use self::dotenv::dotenv;
use self::rustc_serialize::json;
use self::fresh_cargo::*;
use self::fresh_cargo::models::*;
use self::diesel::prelude::*;
use std::env;

#[derive(RustcDecodable, RustcEncodable)]
struct EncodeableCrates {
    pub crate_object: Vec<Crate>
}

fn index(_: &mut Request) -> IronResult<Response> {
    use fresh_cargo::schema::crates::dsl::*;
    let connection = establish_connection();
    let results = crates
        .load::<Crate>(&connection)
        .expect("Error loading crates");

    let encodeable_crate = EncodeableCrates {
        crate_object: results,
    };

    let mut resp = Response::new();
    resp.set_mut(Template::new("index", json::encode(&encodeable_crate).unwrap())).set_mut(status::Ok);
    println!("{}", resp);
    Ok(resp)
}

fn main() {
    dotenv().ok();
    let mut hbse = HandlebarsEngine::new();

    hbse.add(Box::new(DirectorySource::new("./templates/", ".hbs")));

    if let Err(r) = hbse.reload() {
        panic!("Failed to load handlebars");
    }

    let mut router = Router::new();
    router.get("/", index);
    let mut chain = Chain::new(router);
    chain.link_after(hbse);
    let url = format!("0.0.0.0:{}", env::var("PORT").unwrap());
    println!("Binding on {:?}", url);
    Iron::new(chain).http(&url[..]).unwrap();
    println!("Bound on {:?}", url);
}
