use crate::adapters::entities::StaffTypeEntity;

#[derive(Deserialize,Serialize,Default,Clone)]
pub struct StaffTypeData {
    pub id: uuid::Uuid,
    pub staff_type:String,
}


impl From<StaffTypeEntity> for StaffTypeData{
    fn from(value: StaffTypeEntity) -> Self {
        return Self{
            id: value.id,
            staff_type: value.staff_type,
        }
    }
}


impl From<StaffTypeData> for StaffTypeEntity{
    fn from(value: StaffTypeData) -> Self {
        return Self{
            id: value.id,
            staff_type: value.staff_type,
        }
    }
}