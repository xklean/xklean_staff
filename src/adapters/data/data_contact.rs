

#[derive(Deserialize,Serialize,Default,Clone)]
pub struct Contact {
    pub id: uuid::Uuid,
    pub contact_type_id:uuid::Uuid,
    pub contact_type:String,
    pub contact:String
}