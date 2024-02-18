use serde::{Deserialize, Serialize};
use crate::adapters::entites::ContactTypeEntity;

#[derive(Deserialize,Serialize,Default,Clone)]
pub struct ContactType {
    pub id: uuid::Uuid,
    pub contact_type:String,
}

impl From<ContactTypeEntity> for ContactType{
    fn from(value: ContactTypeEntity) -> Self {
        return Self{
            id: value.id,
            contact_type: value.contact_type,
        }
    }
}


impl From<ContactType> for ContactTypeEntity{
    fn from(value: ContactType) -> Self {
        return Self{
            id: value.id,
            contact_type: value.contact_type,
        }
    }
}