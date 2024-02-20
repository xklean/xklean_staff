use serde::{Deserialize, Serialize};
use crate::adapters::entites::ContactEntity;

#[derive(Deserialize,Serialize,Default,Clone)]
pub struct ContactData {
    pub id: uuid::Uuid,
    pub contact_type_id:uuid::Uuid,
    pub contact_type:String,
    pub contact:String,
    pub primary:bool
}

impl From<ContactEntity> for ContactData{
    fn from(value: ContactEntity) -> Self {
        return Self{
            id: value.id,
            contact_type_id: value.contact_type_id,
            contact_type: value.contact_type,
            contact: value.contact,
            primary: value.primary,
        }
    }
}

impl From<ContactData> for ContactEntity{
    fn from(value: ContactData) -> Self {
        return Self{
            id: value.id,
            contact_type_id: value.contact_type_id,
            contact_type: value.contact_type,
            contact: value.contact,
            primary: value.primary,
        }
    }
}

