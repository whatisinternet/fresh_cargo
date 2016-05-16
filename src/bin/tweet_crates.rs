extern crate fresh_cargo;
extern crate diesel;
extern crate oauth_client as oauth;
extern crate twitter_api as twitter;
extern crate rustc_serialize;

#[macro_use]
extern crate dotenv;

use self::fresh_cargo::*;
use self::fresh_cargo::models::*;
use self::diesel::prelude::*;
use self::oauth::Token;
use dotenv::dotenv;
use std::env;

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Config {
    pub consumer_key: String,
    pub consumer_secret: String,
    pub access_key: String,
    pub access_secret: String,
}

fn main() {
    dotenv().ok();

    use fresh_cargo::schema::crates::dsl::*;
    let connection = establish_connection();
    let results = crates.filter(published.eq(false))
        .limit(300)
        .load::<Crate>(&connection)
        .expect("Error loading crates");

    let config = Config {
        consumer_key: env::var("TWITTER_CONSUMER_KEY").expect("TWITTER_CONSUMER_KEY must be set"),
        consumer_secret: env::var("TWITTER_CONSUMER_SECRET")
            .expect("TWITTER_CONSUMER_SECRET must be set"),
        access_key: env::var("TWITTER_ACCESS_TOKEN_KEY")
            .expect("TWITTER_ACCESS_TOKEN_KEY must be set"),
        access_secret: env::var("TWITTER_ACCESS_TOKEN_SECRET")
            .expect("TWITTER_ACCESS_TOKEN_SECRET must be set"),
    };

    let consumer = Token::new(config.consumer_key, config.consumer_secret);
    let access = Token::new(config.access_key, config.access_secret);

    for _crate in results {
        let updated_crate = diesel::update(crates.find(_crate.id))
            .set(published.eq(true))
            .get_result::<Crate>(&connection)
            .expect(&format!("Unable to find crate {}", _crate.id));
        let status = &*build_tweet(updated_crate);
        twitter::update_status(&consumer, &access, status).unwrap();
    }
}

fn build_tweet(crate_struct: Crate) -> String {
    let mut tweet = format!("{} ({}) {} {}",
                            crate_struct.name,
                            crate_struct.version,
                            crate_struct.url,
                            crate_struct.description);
    if tweet.len() > 130 {
        tweet.truncate(130);
        tweet = format!("{}..." tweet);
    }
    return tweet
}
