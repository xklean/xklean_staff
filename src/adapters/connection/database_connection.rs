use async_trait::async_trait;
use sea_orm::{DatabaseConnection, DbErr};

#[async_trait]
pub trait IConnection{
    async fn connect(&self,db_url:String)->Result<DatabaseConnection,DbErr>;
}