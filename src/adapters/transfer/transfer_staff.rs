use chrono::NaiveDate;
use uuid::Uuid;
use crate::adapters::entities::StaffEntity;

#[derive(Serialize,Deserialize,Default,Clone)]
pub struct  StaffData{
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
    pub commence_date:Option<NaiveDate>,
    pub operation_user_id:Uuid,
    pub address: Vec<AddressData>,
    pub contacts:Vec<ContactData>
}

impl From<StaffEntity> for StaffData{
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
            address: vec![],
            contacts: vec![],
        }
    }
}

impl From<StaffData> for StaffEntity {
    fn from(value: StaffData) -> Self {
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
        }
    }
}

impl From<&StaffData> for StaffEntity {
    fn from(value: &StaffData) -> Self {
        return Self{
            id: value.id,
            first_name: value.first_name.to_string(),
            last_name: value.last_name.to_string(),
            email_address: value.email_address.to_string(),
            vehicle_registration: value.vehicle_registration.clone(),
            staff_type_id:value.staff_type_id,
            staff_type: value.staff_type.to_string(),
            tenant_id:value.tenant_id,
            sex: value.sex.to_string(),
            hourly_rate: value.hourly_rate,
            active: value.active,
            commence_date: value.commence_date,
            operation_user_id:value.operation_user_id,
        }
    }
}