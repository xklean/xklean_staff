use std::sync::Arc;
use uuid::Uuid;

#[async_trait]
pub trait IMutationRepository :Send+Sync{
    async fn create_staff(&self,staff:Box<Arc<data::Staff>>)->types::Response<Uuid>;
    async fn upsert_contacts(&self,staff_id:Uuid,contacts:Box<Arc<Vec<data::Contact>>>)->types::Response<bool>;
    async fn upsert_address(&self,staff_id:Uuid,address:Box<Arc<Vec<data::Address>>>)->types::Response<bool>;
    async fn upsert_staff_type(&self,staff_type:Box<Arc<data::StaffType>>)->types::Response<bool>;
    async fn upsert_contact_type(&self,contact_type:Box<Arc<data::ContactType>>)->types::Response<bool>;
}