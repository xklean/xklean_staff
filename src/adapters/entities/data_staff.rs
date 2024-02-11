use serde::{Deserialize, Serialize};
use crate::adapters::entities::data_address::Address;
use crate::adapters::entities::data_contact::Contact;

#[derive(Serialize,Deserialize,Default)]
pub struct  Staff{
    pub id:uuid::Uuid,
    pub first_name:String,
    pub last_name:String,
    pub email_address:String,
    pub vehicle_registration:Option<String>,
    pub staff_type_id:uuid::Uuid,
    pub contractor_id:uuid::Uuid,
    pub sex:String,
    pub contacts:Vec<Contact>,
    pub address:Vec<Address>
}