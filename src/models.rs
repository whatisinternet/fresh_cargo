use super::schema::crates;

#[derive(Queryable)]
pub struct Crate {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub description: String,
    pub version: String,
    pub published: bool,
}

#[insertable_into(crates)]
pub struct NewCrate<'a> {
    pub name: &'a str,
    pub url: &'a str,
    pub description: &'a str,
    pub version: &'a str,
}
