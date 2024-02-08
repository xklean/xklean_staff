use crate::config::configuration::Configuration as cfg;

mod adapters;
mod config;

#[tokio::main]
async fn main() {
   let cfg =  cfg::init_config();

    println!("{}",cfg.db_user);
    println!("Hello, world!");
}
