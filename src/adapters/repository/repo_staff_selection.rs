use std::collections::HashMap;
use async_trait::async_trait;
use crate::adapters::types;
use crate::adapters::entities;
#[async_trait]
pub trait ISelectionRepository :Send+Sync{
   async fn get_staff_by_id(&self,tenant_id:Uuid,id:Uuid)->types::Response<entities::StaffEntity>;
   async fn get_contacts_staff_id(&self,tenant_id:Uuid,staff_id:Uuid)->types::Response<Vec<entities::ContactEntity>>;
   async fn get_address_staff_id(&self,tenant_id:Uuid, staff_id: Uuid) -> types::Response<Vec<entities::AddressEntity>>;
   async fn get_staffs_ids(&self,tenant_id:Uuid,ids:Vec<Uuid>)->types::Response<Vec<entities::StaffEntity>>;
   async fn get_contacts_staff_ids(&self,tenant_id:Uuid,staff_ids:Vec<Uuid>)->types::Response<HashMap<String, entities::ContactEntity>>;
   async fn get_address_staff_ids(&self,tenant_id:Uuid,staff_ids:Vec<Uuid>)->types::Response<Vec<entities::AddressEntity>>;
   async fn get_staff_by_name(&self,tenant_id:Uuid ,name:String)->types::Response<Vec<entities::StaffEntity>>;
   async fn get_all_staff_types(&self,tenant_id:Uuid)-> types::Response<Vec<entities::StaffTypeEntity>>;
   async fn get_all_contact_types(&self,tenant_id:Uuid)-> types::Response<Vec<entities::ContactTypeEntity>>;
}