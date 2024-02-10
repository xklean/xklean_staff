use async_trait::async_trait;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use sea_orm::*;

use crate::adapters::connection::IConnection;

#[derive(Default)]
pub struct DataBaseConn();

#[async_trait]
impl IConnection for DataBaseConn{
    async fn connect(&self,db_url: String) -> Result<DatabaseConnection, DbErr> {
        let mut opts=ConnectOptions::new(db_url);
        opts.sqlx_logging(false);
        opts.set_schema_search_path("public");

        return Database::connect(opts).await;
    }
}