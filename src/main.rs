extern crate actix_web;
#[macro_use]
extern crate mysql;
#[macro_use]
extern crate serde_derive;

mod build_config;
mod data_store;
mod model;
mod web_server;


fn main() {
    let config = build_config::create_config();

    let pool = data_store::create_pool(&config);

    web_server::start_server(config, pool).unwrap();
}


