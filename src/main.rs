use crate::config::configuration::Configuration;

mod adapters;
mod config;

#[tokio::main]
async fn main() {
   let cfg =  Configuration::init_config();

    println!("{}",cfg.db_user);
    println!("Hello, world!");
}
