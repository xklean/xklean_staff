use async_trait::async_trait;
use crate::adapters::errors;

#[async_trait]
pub trait IRepository {
   async fn get_staff_by_id(&self,id:uuid::Uuid)->Result<data::Staff, errors::ServiceErr>;
   async fn get_contacts_staff_id(&self,staff_id:uuid::Uuid)->Result<Vec<data::Contact>,errors::ServiceErr>;
}