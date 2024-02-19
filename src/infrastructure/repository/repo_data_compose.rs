use crate::adapters::entites;
use crate::infrastructure::repository::entities::tbl_staff;
use num_traits::cast::ToPrimitive;

impl From<tbl_staff::Model> for entites::StaffEntity{
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