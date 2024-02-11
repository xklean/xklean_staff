use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize,Default)]
pub struct Contact {
    pub id: uuid::Uuid,
    pub contact_type_id:uuid::Uuid,
    pub contact_type:String,
    pub contact:String
}