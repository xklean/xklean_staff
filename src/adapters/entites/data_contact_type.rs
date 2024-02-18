#[derive(Deserialize,Serialize,Default,Clone)]
pub struct ContactTypeEntity {
    pub id: uuid::Uuid,
    pub contact_type:String,
}