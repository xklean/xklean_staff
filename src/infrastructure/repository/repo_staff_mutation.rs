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
        let stf = (**staff).clone();

        let id_func = || -> uuid::Uuid{
            if stf.id == uuid::Uuid::default() {
                return stf.id.clone();
            }

            return Uuid::new_v4();
        };

        let staff_id = id_func();

        let hour_rate = Decimal::from_f32(stf.hourly_rate).unwrap_or(Decimal::new(0, 0));

        let staff_rec = tbl_staff::Entity::find_by_id(staff_id.clone())
            .one(self.conn.as_ref()).await?;

        if let Some(exists)= staff_rec {
            let mut staff_model = exists.into_active_model();

            staff_model.staff_type_id= Set(stf.staff_type_id.to_owned());
            staff_model.active=Set(stf.active.to_owned());
            staff_model.sex=Set(stf.sex.to_owned());
            staff_model.hourly_rate=Set(hour_rate.to_owned());
            staff_model.contractor_id=Set(stf.contractor_id.to_owned());
            staff_model.vehicle_registration=Set(stf.vehicle_registration.to_owned());
            staff_model.email_address=Set(stf.email_address.to_owned());
            staff_model.last_name=Set(stf.last_name.to_owned());
            staff_model.first_name=Set(stf.first_name.to_owned());

            tbl_staff::Entity::update(staff_model)
                .filter(tbl_staff::Column::Id.eq(staff_id.clone()))
                .exec(self.conn.as_ref()).await?;

            return Ok(staff_id)
        }

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
    async fn upsert_contacts(
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

            let rec_contact = tbl_contact::Entity::find_by_id(contact_id.clone())
                .one(self.conn.as_ref()).await?;

            if let Some(exists)= rec_contact {
                let mut contact_model = exists.into_active_model();

                contact_model.contact_value=Set(con.contact_type.to_owned());
                contact_model.contact_type_id=Set(con.contact_type_id.to_owned());

                tbl_contact::Entity::update(contact_model)
                    .filter(tbl_contact::Column::Id.eq(contact_id.clone()))
                    .exec(self.conn.as_ref()).await?;
            }else{
                let contact_model = tbl_contact::ActiveModel {
                    id: Set(contact_id.to_owned()),
                    contact_type_id: Set(con.contact_type_id.to_owned()),
                    contact_value: Set(con.contact.to_owned()),
                };

                tbl_contact::Entity::insert(contact_model)
                    .exec(self.conn.as_ref()).await?;
            }

            let rec_staff_contact = tbl_staff_contact::Entity::find()
                .filter(tbl_staff_contact::Column::StaffId.eq(staff_id.clone())
                    .and(tbl_staff_contact::Column::ContactId.eq(contact_id.clone())))
                .one(self.conn.as_ref()).await?;
            if let Some(exists) = rec_staff_contact {
                let mut staff_contact_model= exists.into_active_model();

                staff_contact_model.contact_type_id=Set(con.contact_type_id.to_owned());
                staff_contact_model.primary = Set(con.primary.to_owned());

                tbl_staff_contact::Entity::update(staff_contact_model)
                    .filter(tbl_staff_contact::Column::StaffId.eq(staff_id.clone())
                        .and(tbl_staff_contact::Column::ContactId.eq(contact_id.clone())))
                    .exec(self.conn.as_ref()).await?;


            }else{
                let staff_contact_model = tbl_staff_contact::ActiveModel {
                    staff_id: Set(staff_id.to_owned()),
                    contact_type_id: Set(con.contact_type_id.to_owned()),
                    primary: Set(con.primary.to_owned()),
                    contact_id: Set(contact_id.to_owned()),
                };

                tbl_staff_contact::Entity::insert(staff_contact_model)
                    .exec(self.conn.as_ref()).await?;
            }
        }

        Ok(true)
    }

    //---------------------------------------------------------------------------
    //create or update address
    //---------------------------------------------------------------------------
    async fn upsert_address(
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

            let rec_address = tbl_address::Entity::find_by_id(address_id.clone())
                .one(self.conn.as_ref()).await?;

            if let Some(exists) = rec_address {
                let mut addr_model = exists.into_active_model();

                addr_model.suburb = Set(add.suburb.to_owned());
                addr_model.country = Set(add.country.to_owned());
                addr_model.state = Set(add.state.to_owned());
                addr_model.post_code = Set(add.post_code.to_owned());
                addr_model.street_name = Set(add.street_name.to_owned());

                tbl_address::Entity::update(addr_model).filter(tbl_address::Column::Id
                    .eq(address_id.clone()))
                    .exec(self.conn.as_ref()).await?;
            } else {
                let address_model = tbl_address::ActiveModel {
                    id: Set(address_id.to_owned()),
                    street_name: Set(add.street_name.to_owned()),
                    suburb: Set(add.suburb.to_owned()),
                    post_code: Set(add.post_code.to_owned()),
                    state: Set(add.state.to_owned()),
                    country: Set(add.country.to_owned()),
                };

                tbl_address::Entity::insert(address_model)
                    .exec(self.conn.as_ref()).await?;
            }

            let rec_stf_address = tbl_staff_address::Entity::find()
                .filter(tbl_staff_address::Column::StaffId.eq(staff_id.clone())
                    .and(tbl_staff_address::Column::AddressId.eq(address_id.clone())))
                .one(self.conn.as_ref()).await?;

            if let Some(exists) = rec_stf_address {
                let staff_address_id = exists.id.clone();

                let mut stf_address_model = exists.into_active_model();

                stf_address_model.primary = Set(add.primary.to_owned());

                tbl_staff_address::Entity::update(stf_address_model)
                    .filter(tbl_staff_address::Column::Id.eq(staff_address_id))
                    .exec(self.conn.as_ref()).await?;
            }else{
                let staff_address_model = tbl_staff_address::ActiveModel {
                    id: Set(Uuid::new_v4().to_owned()),
                    staff_id: Set(staff_id.to_owned()),
                    address_id: Set(address_id.to_owned()),
                    primary: Set(add.primary.to_owned()),
                };

                tbl_staff_address::Entity::insert(staff_address_model)
                .exec(self.conn.as_ref()).await?;
            }
        }

        Ok(true)
    }


    //---------------------------------------------------------------------------
    //create or update staff type
    //---------------------------------------------------------------------------
    async fn upsert_staff_type(
        &self,
        staff_type: Box<Arc<StaffType>>) -> Response<bool> {
        let stf_type = &**staff_type;

        let id_func = || -> uuid::Uuid{
            if stf_type.id == uuid::Uuid::default() {
                return stf_type.id.clone();
            }

            return Uuid::new_v4();
        };

        let staff_type_id = id_func();

        let rec_staff_type = tbl_staff_type::Entity::find_by_id(staff_type_id
            .clone()).one(self.conn.as_ref()).await?;

        if let Some(exist) = rec_staff_type {
            let mut stf_type_model = exist.into_active_model();

            stf_type_model.type_name = Set(stf_type.staff_type.to_owned());

            tbl_staff_type::Entity::update(stf_type_model)
                .filter(tbl_staff_type::Column::Id.eq(staff_type_id.clone()))
                .exec(self.conn.as_ref()).await?;

            return Ok(true);
        }

        let staff_type_model = tbl_staff_type::ActiveModel {
            id: Set(staff_type_id.to_owned()),
            type_name: Set(stf_type.staff_type.to_owned()),
        };

        tbl_staff_type::Entity::insert(staff_type_model)
            .exec(self.conn.as_ref()).await?;

        Ok(true)
    }

    //---------------------------------------------------------------------------
    //create or update contact type
    //---------------------------------------------------------------------------
    async fn upsert_contact_type(
        &self,
        contact_type: Box<Arc<ContactType>>) -> Response<bool> {
        let cont_type = &**contact_type;

        let id_func = || -> uuid::Uuid{
            if cont_type.id == uuid::Uuid::default() {
                return cont_type.id.clone();
            }

            return Uuid::new_v4();
        };

        let contact_type_id = id_func();

        let rec_con_type = tbl_contact_type::Entity::find_by_id(contact_type_id
            .clone()).one(self.conn.as_ref()).await?;

        if let Some(exists) = rec_con_type {
            let mut con_type_model = exists.into_active_model();

            con_type_model.contact_type = Set(cont_type.contact_type.to_owned());

            tbl_contact_type::Entity::update(con_type_model)
                .filter(tbl_contact_type::Column::Id.eq(contact_type_id.clone()))
                .exec(self.conn.as_ref()).await?;

            return Ok(true);
        }


        let contact_type_model = tbl_contact_type::ActiveModel {
            id: Set(id_func().to_owned()),
            contact_type: Set(cont_type.contact_type.to_owned()),
        };

        tbl_contact_type::Entity::insert(contact_type_model)
            .exec(self.conn.as_ref()).await?;

        Ok(true)
    }
}