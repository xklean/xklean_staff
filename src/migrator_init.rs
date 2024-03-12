mod migrate;
mod config;
mod adapters;
mod infra;

use sea_orm_migration::{MigratorTrait};
use adapters::connection::{IConnection};
use crate::config::configuration::Configuration as cfg;
use crate::infra::connection::DataBaseConn;
use crate::migrate::Migrator;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg_default = cfg::init_config();

    let conn_url = cfg_default.get_connection_url();
    let conn = DataBaseConn::default();
    let db = match conn.connect(conn_url).await{
        Ok(db)=> db,
        Err(err)=>panic!("{}",err)
    };

    match Migrator::up(&db,None).await {
        Err(err) => panic!("{}", err),
        Ok(_) => ()
    }

    Ok(())
}