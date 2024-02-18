use sea_orm::prelude::DateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::adapters::entites::StaffEntity;



#[derive(Serialize,Deserialize,Default,Clone)]
pub struct  Staff{
    pub id:uuid::Uuid,
    pub first_name:String,
    pub last_name:String,
    pub email_address:String,
    pub vehicle_registration:Option<String>,
    pub staff_type_id:uuid::Uuid,
    pub staff_type:String,
    pub tenant_id:uuid::Uuid,
    pub sex:String,
    pub hourly_rate:f32,
    pub active:bool,
    pub commence_date:DateTime,
    pub operation_user_id:Uuid,
    pub created_at:DateTime,
    pub updated_at:Option<DateTime>,
    pub deleted_at:Option<DateTime>
}

impl From<StaffEntity> for Staff{
    fn from(value: StaffEntity) -> Self {
        return Self{
            id: value.id,
            first_name: value.first_name,
            last_name: value.last_name,
            email_address: value.email_address,
            vehicle_registration: value.vehicle_registration,
            staff_type_id:value.staff_type_id,
            staff_type: value.staff_type,
            tenant_id:value.tenant_id,
            sex: value.sex,
            hourly_rate: value.hourly_rate,
            active: value.active,
            commence_date: value.commence_date,
            operation_user_id:value.operation_user_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
            deleted_at: value.deleted_at,
        }
    }
}

impl From<Staff> for StaffEntity {
    fn from(value: Staff) -> Self {
        return Self{
            id: value.id,
            first_name: value.first_name,
            last_name: value.last_name,
            email_address: value.email_address,
            vehicle_registration: value.vehicle_registration,
            staff_type_id:value.staff_type_id,
            staff_type: value.staff_type,
            tenant_id:value.tenant_id,
            sex: value.sex,
            hourly_rate: value.hourly_rate,
            active: value.active,
            commence_date: value.commence_date,
            operation_user_id:value.operation_user_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
            deleted_at: value.deleted_at,
        }
    }
}