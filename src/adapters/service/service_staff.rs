use async_trait::async_trait;
use uuid::Uuid;
use crate::adapters::transfer::{AddressData, ContactData, ContactTypeData, StaffData, StaffTypeData};
use crate::adapters::types::Response;

#[async_trait]
pub trait IStaffService :Sync +Send {
    async fn get_staff_by_staff_id(&self,staff_id:Uuid)-> Response<StaffData>;
    async fn get_staff_by_first_name(&self,staff_id:Uuid)-> Response<Vec<StaffData>>;
    async fn get_address_by_staff_id(&self,staff_id:Uuid)-> Response<Vec<AddressData>>;
    async fn get_contact_by_staff_id(&self,staff_id:Uuid)-> Response<Vec<ContactData>>;
    async fn get_all_staff_types(&self)-> Response<Vec<StaffTypeData>>;
    async fn get_all_contact_types(&self)-> Response<Vec<ContactTypeData>>;
    async fn upsert_staff(&self,staff:StaffData)-> Response<bool>;
}