use crate::adapters::entities;
use crate::infrastructure::repository::entities::{
    tbl_staff,
    tbl_contact,
    tbl_address,
    tbl_contact_type,
    tbl_staff_type};
use num_traits::cast::ToPrimitive;
use crate::infrastructure::repository::entities::tbl_contact::Model;

impl From<tbl_staff::Model> for entities::StaffEntity{
    fn from(staff: tbl_staff::Model) -> Self {
        let hour_rate = staff.hourly_rate.to_f32().unwrap_or_else(|| 0.0);
        return Self{
            id: staff.id,
            first_name: staff.first_name,
            last_name: staff.last_name,
            email_address: staff.email_address,
            vehicle_registration: staff.vehicle_registration,
            staff_type_id: staff.staff_type_id,
            staff_type: "".to_string(),
            tenant_id: staff.tenant_id,
            sex: staff.sex,
            hourly_rate: hour_rate,
            active: staff.active,
            commence_date: staff.commence_date,
            operation_user_id: staff.operation_user_id,
            created_at: staff.created_at,
            updated_at: staff.updated_at,
            deleted_at: None,
        }
    }
}

impl From<tbl_contact::Model> for entities::ContactEntity  {
    fn from(value: tbl_contact::Model) -> Self {
        return Self{
            id: value.id,
            contact_type_id: value.contact_type_id,
            contact_type: "".to_string(),
            contact: value.contact_value,
            primary: false,
        }
    }
}

impl From<tbl_address::Model> for entities::AddressEntity {
    fn from(value: tbl_address::Model) -> Self {
        return Self{
            id: value.id,
            street_name:value.street_name,
            suburb: value.suburb,
            post_code:value.post_code,
            state: value.state,
            country: value.country,
            primary: false,
        }
    }
}

impl From<tbl_contact_type::Model> for entities::ContactTypeEntity {
    fn from(value: tbl_contact_type::Model) -> Self {
        return Self{
            id: value.id,
            contact_type: value.contact_type,
        }
    }
}

impl From<tbl_staff_type::Model> for entities::StaffTypeEntity {
    fn from(value: tbl_staff_type::Model) -> Self {
        return Self{
            id: value.id,
            staff_type:value.type_name,
        }
    }
}