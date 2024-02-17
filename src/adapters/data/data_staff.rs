use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Default,Clone)]
pub struct  Staff{
    pub id:uuid::Uuid,
    pub first_name:String,
    pub last_name:String,
    pub email_address:String,
    pub vehicle_registration:Option<String>,
    pub staff_type_id:uuid::Uuid,
    pub contractor_id:uuid::Uuid,
    pub sex:String,
    pub hourly_rate:f32,
    pub active:bool,
    pub contacts:Vec<Contact>,
    pub address:Vec<Address>
}