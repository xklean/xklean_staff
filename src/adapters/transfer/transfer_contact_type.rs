use serde::{Deserialize, Serialize};
use crate::adapters::entites::ContactTypeEntity;

#[derive(Deserialize,Serialize,Default,Clone)]
pub struct ContactTypeData {
    pub id: uuid::Uuid,
    pub contact_type:String,
}

impl From<ContactTypeEntity> for ContactTypeData{
    fn from(value: ContactTypeEntity) -> Self {
        return Self{
            id: value.id,
            contact_type: value.contact_type,
        }
    }
}


impl From<ContactTypeData> for ContactTypeEntity{
    fn from(value: ContactTypeData) -> Self {
        return Self{
            id: value.id,
            contact_type: value.contact_type,
        }
    }
}