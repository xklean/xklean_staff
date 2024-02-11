use std::sync::Arc;
use sea_orm::DatabaseConnection;

pub struct Repository{
    pub conn:Box<Arc<DatabaseConnection>>
}

impl Repository {

}