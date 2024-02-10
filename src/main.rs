#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

use crate::config::configuration::Configuration as cfg;

mod adapters;
mod config;
mod application;
mod infrastructure;


pub mod pb_staff {
    include!("../server/serverstaff.rs");
}

#[tokio::main]
async fn main() {
   let cfg =  cfg::init_config();

    println!("{}",cfg.db_user);
    println!("Hello, world!");
}


