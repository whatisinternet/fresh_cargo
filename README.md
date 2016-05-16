# Fresh cargo

A twitter bot to publish the newest rust crates!

Features:
---------

- Fetch latest and updated crates from crates.io
- List database entries to screen
- Tweet collection from database
- Run web server (iron) with ReactJS front-end

## Setup

**Requires PostgreSQL**
**Requires Twitter developer tokens**

1. Install [Rustup](https://www.rustup.rs/)
2. `rustup toolchain install nightly`
3. `cargo install rustfmt`
4. `cargo install diesel_cli`
5. `echo DATABASE_URL=postgres://localhost/fresh_cargo >> .env`
6. `echo TWITTER_CONSUMER_KEY=[KEY HERE] >> .env`
7. `echo TWITTER_CONSUMER_SECRET=[KEY HERE] >> .env`
8. `echo TWITTER_ACCESS_TOKEN_KEY=[KEY HERE] >> .env`
9. `echo TWITTER_ACCESS_TOKEN_SECRET=[KEY HERE] >> .env`
10. `diesel setup`
11. `diesel migration run`
12. `cargo build`

## Run

### Write crates to db

```shell
  ./target/debug/write_crates
```

### Read crates from the db

```shell
  ./target/debug/show_crates
```

### Web server

```shell
  ./target/debug/web
```

### Tweet untweeted crates

```shell
  ./target/debug/tweet_crates
```

## Contributing

This project is intended to be a safe, welcoming space for collaboration, and
contributors are expected to adhere to the [Contributor Covenant](http://contributor-covenant.org) code of conduct.

1. Fork it ( https://github.com/whatisinternet/fresh_cargo/fork )
2. Create your feature branch (`git checkout -b my-new-feature`)
3. Commit your changes (`git commit -am 'Add some feature'`)
4. Push to the branch (`git push origin my-new-feature`)
5. Create a new Pull Request
