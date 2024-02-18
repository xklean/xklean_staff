use serde::{Deserialize, Serialize};
use crate::adapters::entites::StaffTypeEntity;

#[derive(Deserialize,Serialize,Default,Clone)]
pub struct StaffType {
    pub id: uuid::Uuid,
    pub staff_type:String,
}


impl From<StaffTypeEntity> for StaffType{
    fn from(value: StaffTypeEntity) -> Self {
        return Self{
            id: value.id,
            staff_type: value.staff_type,
        }
    }
}


impl From<StaffType> for StaffTypeEntity{
    fn from(value: StaffType) -> Self {
        return Self{
            id: value.id,
            staff_type: value.staff_type,
        }
    }
}