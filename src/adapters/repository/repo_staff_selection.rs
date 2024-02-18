use std::collections::HashMap;
use async_trait::async_trait;
use crate::adapters::types;
use crate::adapters::entites;
#[async_trait]
pub trait ISelectionRepository :Send+Sync{
   async fn get_staff_by_id(&self,id:uuid::Uuid)->types::Response<entites::StaffEntity>;
   async fn get_contacts_staff_id(&self,staff_id:uuid::Uuid)->types::Response<Vec<entites::ContactEntity>>;
   async fn get_address_staff_id(&self, staff_id: uuid::Uuid) -> types::Response<Vec<entites::AddressEntity>>;
   async fn get_staffs_ids(&self,ids:Vec<uuid::Uuid>)->types::Response<Vec<entites::StaffEntity>>;
   async fn get_contacts_staff_ids(&self,staff_ids:Vec<uuid::Uuid>)->types::Response<HashMap<String, entites::ContactEntity>>;
   async fn get_address_staff_ids(&self,staff_ids:Vec<uuid::Uuid>)->types::Response<Vec<entites::AddressEntity>>;
}