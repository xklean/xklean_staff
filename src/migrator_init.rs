mod migrate;
mod config;
mod adapters;
mod infrastructure;

use adapters::connection::{IConnection};
use crate::config::configuration::Configuration as cfg;
use crate::infrastructure::connection::DataBaseConn;

#[tokio::main]
 async fn main()-> Result<(),Box<dyn std::error::Error>>{
   let cfg_default = cfg::init_config();
   let conn =DataBaseConn::default();

   conn.connect(cfg_default.)

  Ok(())
 }