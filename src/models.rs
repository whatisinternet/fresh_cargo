use super::schema::crates;
use super::diesel::*;

#[derive(Queryable)]
#[derive(RustcDecodable, RustcEncodable)]
#[changeset_for(crates)]
pub struct Crate {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub description: String,
    pub version: String,
    pub published: bool,
}

#[insertable_into(crates)]
#[changeset_for(crates)]
pub struct NewCrate<'a> {
    pub name: &'a str,
    pub url: &'a str,
    pub description: &'a str,
    pub version: &'a str,
    pub published: bool,
}
