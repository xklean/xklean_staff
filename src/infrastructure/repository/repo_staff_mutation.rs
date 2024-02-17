use num_traits::FromPrimitive;
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::Decimal;
use crate::adapters::data::{ContactType, Staff, StaffType};
use crate::adapters::repository::IMutationRepository;
use crate::adapters::types::Response;

#[async_trait]
impl IMutationRepository for Repository {
    //---------------------------------------------------------------------------
    //create staff
    //---------------------------------------------------------------------------
    async fn create_staff(
        &self,
        staff: Box<Arc<Staff>>) -> types::Response<Uuid> {
        let staff_id = Uuid::new_v4();

        let stf = (**staff).clone();


        let hour_rate = Decimal::from_f32(stf.hourly_rate).unwrap_or(Decimal::new(0, 0));

        let staff_model = tbl_staff::ActiveModel {
            id: Set(staff_id.to_owned()),
            first_name: Set(stf.first_name.to_owned()),
            last_name: Set(stf.last_name.to_owned()),
            email_address: Set(stf.email_address.to_owned()),
            vehicle_registration: Set(stf.vehicle_registration.to_owned()),
            staff_type_id: Set(stf.staff_type_id.to_owned()),
            contractor_id: Set(stf.contractor_id.to_owned()),
            sex: Set(stf.sex.to_owned()),
            active: Set(stf.active.to_owned()),
            hourly_rate: Set(hour_rate),
        };

        tbl_staff::Entity::insert(staff_model)
            .exec(self.conn.as_ref()).await?;


        Ok(staff_id)
    }

    //---------------------------------------------------------------------------
    //create contact
    //---------------------------------------------------------------------------
    async fn create_contacts(
        &self,
        staff_id: Uuid,
        contacts: Box<Arc<Vec<Contact>>>) -> Response<bool> {
        let contacts_list = &**contacts;

        for con in contacts_list {
            let id_func = || -> uuid::Uuid{
                if con.id == uuid::Uuid::default() {
                    return con.id.clone();
                }

                return Uuid::new_v4();
            };

            let contact_id = id_func();

            let contact_model = tbl_contact::ActiveModel {
                id: Set(contact_id.to_owned()),
                contact_type_id: Set(con.contact_type_id.to_owned()),
                contact_value: Set(con.contact.to_owned()),
            };

            tbl_contact::Entity::insert(contact_model)
                .exec(self.conn.as_ref()).await?;


            let staff_contact_model = tbl_staff_contact::ActiveModel {
                staff_id: Set(staff_id.to_owned()),
                contact_type_id: Set(con.contact_type_id.to_owned()),
                primary: Set(con.primary.to_owned()),
                contact_id: Set(contact_id.to_owned()),
            };

            tbl_staff_contact::Entity::insert(staff_contact_model)
                .exec(self.conn.as_ref()).await?;
        }

        Ok(true)
    }

    //---------------------------------------------------------------------------
    //create address
    //---------------------------------------------------------------------------
    async fn create_address(
        &self,
        staff_id: Uuid,
        address: Box<Arc<Vec<Address>>>) -> Response<bool> {
        let address = &**address;

        for add in address {
            let id_address_func = || -> uuid::Uuid{
                if add.id == uuid::Uuid::default() {
                    return add.id.clone();
                }

                return Uuid::new_v4();
            };

            let address_id = id_address_func();

            let address_model = tbl_address::ActiveModel {
                id: Set(address_id.to_owned()),
                street_name: Set(add.street_name.to_owned()),
                suberb: Set(add.suburb.to_owned()),
                post_code: Set(add.post_code.to_owned()),
                state: Set(add.state.to_owned()),
                country: Set(add.country.to_owned()),
            };

            tbl_address::Entity::insert(address_model)
                .exec(self.conn.as_ref()).await?;


            let staff_address_model = tbl_staff_address::ActiveModel {
                id: Set(Uuid::new_v4().to_owned()),
                staff_id: Set(staff_id.to_owned()),
                address_id: Set(address_id.to_owned()),
                primary: Set(add.primary.to_owned()),
            };

            tbl_staff_address::Entity::insert(staff_address_model)
                .exec(self.conn.as_ref()).await?;
        }

        Ok(true)
    }

    //---------------------------------------------------------------------------
    //create staff type
    //---------------------------------------------------------------------------
    async fn create_staff_type(
        &self,
        staff_type: Box<Arc<StaffType>>) -> Response<bool> {
        let id_func = || -> uuid::Uuid{
            if staff_type.id == uuid::Uuid::default() {
                return staff_type.id.clone();
            }

            return Uuid::new_v4();
        };

        let staff_type_model = tbl_staff_type::ActiveModel {
            id: Set(id_func()),
            type_name: Set(staff_type.staff_type.to_owned()),
        };


        tbl_staff_type::Entity::insert(staff_type_model)
            .exec(self.conn.as_ref()).await?;

        Ok(true)
    }

    //---------------------------------------------------------------------------
    //create contact type
    //---------------------------------------------------------------------------
    async fn create_contact_type(
        &self,
        contact_type: Box<Arc<ContactType>>) -> Response<bool> {
        let id_func = || -> uuid::Uuid{
            if contact_type.id == uuid::Uuid::default() {
                return contact_type.id.clone();
            }

            return Uuid::new_v4();
        };

        let contact_type_model = tbl_contact_type::ActiveModel{
            id: Set(id_func().to_owned()),
            contact_type: Set(contact_type.contact_type.to_owned()),
        };

        tbl_contact_type::Entity::insert(contact_type_model).exec(self.conn.as_ref()).await?;

        Ok(true)
    }
}