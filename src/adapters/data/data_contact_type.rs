#[derive(Deserialize,Serialize,Default,Clone)]
pub struct ContactType {
    pub id: uuid::Uuid,
    pub contact_type:String,
}