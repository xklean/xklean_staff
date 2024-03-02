use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize,Deserialize,Default,Clone)]
pub struct StaffEntity {
    pub id:Uuid,
    pub first_name:String,
    pub last_name:String,
    pub email_address:String,
    pub vehicle_registration:Option<String>,
    pub staff_type_id:Uuid,
    pub staff_type:String,
    pub tenant_id:Uuid,
    pub sex:String,
    pub hourly_rate:f32,
    pub active:bool,
    pub commence_date:Option<NaiveDate>,
    pub operation_user_id:Uuid,
}