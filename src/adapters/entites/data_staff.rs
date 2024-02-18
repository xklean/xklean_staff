use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize,Deserialize,Default,Clone)]
pub struct StaffEntity {
    pub id:uuid::Uuid,
    pub first_name:String,
    pub last_name:String,
    pub email_address:String,
    pub vehicle_registration:Option<String>,
    pub staff_type_id:uuid::Uuid,
    pub staff_type:String,
    pub tenant_id:uuid::Uuid,
    pub sex:String,
    pub hourly_rate:f32,
    pub active:bool,
    pub commence_date:DateTime,
    pub operation_user_id:Uuid,
    pub created_at:DateTime,
    pub updated_at:Option<DateTime>,
    pub deleted_at:Option<DateTime>
}