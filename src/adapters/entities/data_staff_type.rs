
#[derive(Deserialize,Serialize,Default,Clone)]
pub struct StaffTypeEntity {
    pub id: uuid::Uuid,
    pub staff_type:String,
}