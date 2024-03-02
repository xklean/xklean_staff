use async_trait::async_trait;
use uuid::Uuid;
use crate::adapters::transfer::{AddressData, ContactData, ContactTypeData, StaffData, StaffTypeData};
use crate::adapters::types::Response;

#[async_trait]
pub trait IStaffService :Sync +Send {
    async fn get_staff_by_staff_id(&self,tenant_id:Uuid, staff_id:Uuid)-> Response<StaffData>;
    async fn get_staff_by_first_name(&self,tenant_id:Uuid,staff_first_name:String)-> Response<Vec<StaffData>>;
    async fn get_address_by_staff_id(&self,tenant_id:Uuid,staff_id:Uuid)-> Response<Vec<AddressData>>;
    async fn get_contact_by_staff_id(&self,tenant_id:Uuid,staff_id:Uuid)-> Response<Vec<ContactData>>;
    async fn get_all_staff_types(&self,tenant_id:Uuid,)-> Response<Vec<StaffTypeData>>;
    async fn get_all_contact_types(&self,tenant_id:Uuid,)-> Response<Vec<ContactTypeData>>;
    async fn get_staffs_by_ids(&self,tenant_id:Uuid,ids:Vec<uuid::Uuid>)->Response<Vec<StaffData>>;
    async fn upsert_staff(&self,tenant_id:Uuid,staff:StaffData)-> Response<bool>;
    async fn upsert_address(&self,tenant_id:Uuid,staff_id: Uuid,address:AddressData)->Response<bool>;
    async fn upsert_contact(&self,tenant_id:Uuid,staff_id: Uuid,contact:ContactData)->Response<bool>;
    async fn upsert_contact_type(&self,tenant_id:Uuid,contact_type:ContactTypeData)->Response<bool>;
    async fn upsert_staff_type(&self,tenant_id:Uuid,staff_type:StaffTypeData)->Response<bool>;
}