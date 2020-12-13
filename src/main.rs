use env_logger::Env;
use log::debug;

#[macro_use]
extern crate serde_derive;

mod config;

fn init_logger() {
    let env = Env::default()
        .filter_or("LOG_LEVEL", "debug")
        .write_style_or("LOG_STYLE", "always");

    env_logger::init_from_env(env);
}

fn main() {
    init_logger();

    let config = config::get_config().unwrap();
    debug!("env configs: {:?}", config);
}