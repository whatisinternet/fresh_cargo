extern crate iron;
extern crate router;
extern crate handlebars_iron as hbs;
extern crate dotenv;
extern crate rustc_serialize;
extern crate fresh_cargo;
extern crate diesel;
extern crate staticfile;
extern crate mount;

use self::iron::prelude::*;
use self::iron::status;
use self::router::Router;
use self::hbs::{Template, HandlebarsEngine, DirectorySource, MemorySource};
use self::dotenv::dotenv;
use self::rustc_serialize::json;
use self::fresh_cargo::*;
use self::fresh_cargo::models::*;
use self::diesel::prelude::*;
use std::env;
use std::path::Path;
use self::staticfile::Static;
use self::mount::Mount;

#[derive(RustcDecodable, RustcEncodable)]
struct EncodeableCrates {
    pub crate_object: Vec<Crate>,
}

fn index(_: &mut Request) -> IronResult<Response> {
    use fresh_cargo::schema::crates::dsl::*;
    let connection = establish_connection();
    let results = crates.load::<Crate>(&connection)
        .expect("Error loading crates");

    let encodeable_crate = EncodeableCrates { crate_object: results };

    let mut resp = Response::new();
    resp.set_mut(Template::new("index", json::encode(&encodeable_crate).unwrap()))
        .set_mut(status::Ok);
    println!("{}", resp);
    Ok(resp)
}

fn feed(_: &mut Request) -> IronResult<Response> {
    use fresh_cargo::schema::crates::dsl::*;
    let connection = establish_connection();
    let results = crates.load::<Crate>(&connection)
        .expect("Error loading crates");
    let encodeable_crate = EncodeableCrates { crate_object: results };
    let mut resp = Response::new();
    resp.set_mut(json::encode(&encodeable_crate).unwrap().to_owned()).set_mut(status::Ok);
    return Ok(resp);
}

fn main() {
    dotenv().ok();
    let mut hbse = HandlebarsEngine::new();

    hbse.add(Box::new(DirectorySource::new("./web/views/", ".hbs")));

    if let Err(r) = hbse.reload() {
        panic!("Failed to load handlebars");
    }

    let mut router = Router::new();
    router.get("/", index);
    router.get("/feed", feed);



    let mut chain = Chain::new(router);
    chain.link_after(hbse);

    let mut mount = Mount::new();
    mount.mount("/", chain)
        .mount("/assets/", Static::new(Path::new("./web/assets")))
        .mount("/js/", Static::new(Path::new("./web/assets/js")))
        .mount("/css/", Static::new(Path::new("./web/assets/css")))
        .mount("/vendor/", Static::new(Path::new("./web/vendor")));

    let url = format!("0.0.0.0:{}", env::var("PORT").unwrap());
    println!("Binding on {:?}", url);
    Iron::new(mount).http(&url[..]).unwrap();
    println!("Bound on {:?}", url);
}
