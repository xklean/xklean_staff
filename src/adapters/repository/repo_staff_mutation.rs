use std::sync::Arc;
use uuid::Uuid;

#[async_trait]
pub trait IMutationRepository :Send+Sync{
    async fn upsert_staff(&self,tenant_id:Uuid, staff:Box<Arc<entities::StaffEntity>>) ->Response<Uuid>;
    async fn upsert_contacts(&self,tenant_id:Uuid, staff_id:Uuid, contacts:Box<Arc<Vec<entities::ContactEntity>>>) ->Response<bool>;
    async fn upsert_address(&self,tenant_id:Uuid, staff_id:Uuid, address:Box<Arc<Vec<entities::AddressEntity>>>) ->Response<bool>;
    async fn upsert_staff_type(&self,tenant_id:Uuid, staff_type:Box<Arc<entities::StaffTypeEntity>>) ->Response<bool>;
    async fn upsert_contact_type(&self,tenant_id:Uuid, contact_type:Box<Arc<entities::ContactTypeEntity>>) ->Response<bool>;
}