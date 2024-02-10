use sea_orm::{DatabaseConnection, DbErr};
use crate::adapters::connection::IConnection;

#[derive(Default)]
pub struct DBConn();

impl IConnection for DBConn{
    async fn connect(db_url: String) -> Result<DatabaseConnection, DbErr> {
        todo!()
    }
}