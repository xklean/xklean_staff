
#[derive(Deserialize,Serialize,Default,Clone)]
pub struct StaffType {
    pub id: uuid::Uuid,
    pub staff_type:String,
}