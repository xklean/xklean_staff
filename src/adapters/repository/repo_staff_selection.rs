use std::collections::HashMap;
use async_trait::async_trait;
use config::ValueKind::String;
use crate::adapters::errors;
use crate::adapters::types;
use crate::adapters::data;
#[async_trait]
pub trait ISelectionRepository :Send+Sync{
   async fn get_staff_by_id(&self,id:uuid::Uuid)->types::Response<data::Staff>;
   async fn get_contacts_staff_id(&self,staff_id:uuid::Uuid)->types::Response<Vec<data::Contact>>;
   async fn get_address_staff_id(&self, staff_id: uuid::Uuid) -> types::Response<Vec<data::Address>>;
   async fn get_staffs_ids(&self,ids:Vec<uuid::Uuid>)->types::Response<Vec<data::Staff>>;
   async fn get_contacts_staff_ids(&self,staff_ids:Vec<uuid::Uuid>)->types::Response<HashMap<String,data::Contact>>;
   async fn get_address_staff_ids(&self,staff_ids:Vec<uuid::Uuid>)->types::Response<Vec<data::Address>>;
}