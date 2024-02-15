use sea_orm::ActiveValue::Set;
use crate::adapters::data::Staff;
use crate::adapters::repository;
use crate::adapters::repository::IMutationRepository;


impl IMutationRepository for Repository{
    async fn create_staff() -> types::Response<String> {
       let staff_id= Uuid::new_v4();

       let staff_model= tbl_staff::ActiveModel{
           id: Set(staff_id.to_owned()),
           first_name: Default::default(),
           last_name: Default::default(),
           email_address: Default::default(),
           vehicle_registration: Default::default(),
           staff_type_id: Default::default(),
           contractor_id: Default::default(),
           sex: Default::default(),
       };

       Ok(staff_id.to_string())
    }
}