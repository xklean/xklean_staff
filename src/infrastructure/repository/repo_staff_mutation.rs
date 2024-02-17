use num_traits::FromPrimitive;
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::Decimal;
use crate::adapters::data::Staff;
use crate::adapters::repository::IMutationRepository;
use crate::adapters::types::Response;

#[async_trait]
impl IMutationRepository for Repository {
    //---------------------------------------------------------------------------
    //create staff
    //---------------------------------------------------------------------------
    async fn create_staff(&self,staff:Box<Arc<Staff>>) -> types::Response<Uuid> {
        let staff_id = Uuid::new_v4();

        let stf= (**staff).clone();


        let hour_rate =Decimal::from_f32(stf.hourly_rate).unwrap_or(Decimal::new(0, 0));

        let staff_model = tbl_staff::ActiveModel {
            id: Set(staff_id.to_owned()),
            first_name:Set(stf.first_name.to_owned()),
            last_name: Set(stf.last_name.to_owned()),
            email_address: Set(stf.email_address.to_owned()),
            vehicle_registration: Set(stf.vehicle_registration.to_owned()),
            staff_type_id:Set(stf.staff_type_id.to_owned()),
            contractor_id:Set(stf.contractor_id.to_owned()),
            sex: Set(stf.sex.to_owned()),
            active: Set(stf.active.to_owned()),
            hourly_rate:Set(hour_rate),
        };

        let res = tbl_staff::Entity::insert(staff_model)
            .exec(self.conn.as_ref()).await?;

        Ok(staff_id.clone())
    }

    async fn create_contacts(&self, staff_id: Uuid, contacts: Box<Arc<Vec<Contact>>>) -> Response<bool> {
        todo!()
    }

    async fn create_address(&self, staff_id: Uuid, address: Box<Arc<Vec<Address>>>) -> Response<bool> {
        todo!()
    }

}