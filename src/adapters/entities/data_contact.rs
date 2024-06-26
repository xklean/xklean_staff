

#[derive(Deserialize,Serialize,Default,Clone)]
pub struct ContactEntity {
    pub id: uuid::Uuid,
    pub contact_type_id:uuid::Uuid,
    pub contact_type:String,
    pub contact:String,
    pub primary:bool
}