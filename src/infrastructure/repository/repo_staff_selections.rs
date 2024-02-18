use std::collections::HashMap;
use std::sync::{Arc};
use async_trait::async_trait;
use sea_orm::{DatabaseConnection, entity::*, query::*};
use uuid::Uuid;
use data::Contact;
use crate::adapters::repository::ISelectionRepository;
use entities::{prelude::*,
               tbl_staff,
               tbl_contact,
               tbl_contact_type,
               tbl_staff_address,
               tbl_address};
use crate::adapters::{data};
use crate::adapters::data::{Address};
use crate::adapters::errors::ServiceErr;
use crate::infrastructure::repository::entities::{tbl_staff_contact,tbl_staff_type};
use crate::adapters::types;
use num_traits::cast::ToPrimitive;


#[derive(Default)]
pub struct Repository {
    pub conn: Arc<DatabaseConnection>,
}

impl Repository {
    pub fn new(db_conn: Arc<DatabaseConnection>) -> Repository {
        return Repository {
            conn: Arc::clone(&db_conn)
        };
    }
}

#[async_trait]
impl ISelectionRepository for Repository {
    //---------------------------------------------------------------------------
    //get staff by staff id
    //---------------------------------------------------------------------------
    async fn get_staff_by_id(
        &self,
        id: Uuid) -> types::Response<Staff> {
        let staff = TblStaff::find()
            .filter(tbl_staff::Column::Id.eq(id.clone()))
            .one(self.conn.as_ref())
            .await?;

        let data_staff = match staff {
            Some(stf) => {
                let hour_rate = stf.hourly_rate.to_f32().unwrap_or_else(|| 0.0);

                let mut data_result = data::Staff {
                    id,
                    first_name: stf.first_name,
                    last_name: stf.last_name,
                    email_address: stf.email_address,
                    vehicle_registration: stf.vehicle_registration,
                    staff_type_id: stf.staff_type_id,
                    staff_type: "".to_string(),
                    contractor_id: stf.contractor_id,
                    sex: stf.sex,
                    hourly_rate: hour_rate,
                    active: stf.active,
                    contacts: vec![],
                    address: vec![],
                };

                let staff_type = TblStaffType::find()
                    .filter(tbl_staff_type::Column::Id
                        .eq(stf.staff_type_id.clone())).one(self.conn.as_ref()).await?;

                match staff_type {
                    Some(stf_type)=>{
                        data_result.staff_type=stf_type.type_name
                    }
                    None=>()
                }

                Ok(data_result)
            }
            None => {
                Err(ServiceErr("error in data".to_string()))
            }
        };

        return data_staff;
    }

    //---------------------------------------------------------------------------
    //get contact by staff id
    //---------------------------------------------------------------------------
    async fn get_contacts_staff_id(
        &self,
        staff_id: Uuid) -> types::Response<Vec<Contact>> {
        let contacts_staff = TblStaffContact::find()
            .filter(tbl_staff_contact::Column::StaffId.eq(staff_id.clone()))
            .all(self.conn.as_ref())
            .await?;

        let mut contacts: Vec<Contact> = Vec::new();

        for stf_contact in contacts_staff {
            let contact = TblContact::find()
                .filter(tbl_contact::Column::Id.eq(stf_contact.contact_id.clone()))
                .one(self.conn.as_ref())
                .await?;

            let mut data_contact: Contact = Contact::default();
            match contact {
                Some(con) => {
                    data_contact.id = con.id;
                    data_contact.contact = con.contact_value
                }
                None => continue
            }

            let contact_type = TblContactType::find()
                .filter(tbl_contact_type::Column::Id.eq(stf_contact.contact_type_id.clone()))
                .one(self.conn.as_ref())
                .await?;

            match contact_type {
                Some(con_type) => {
                    data_contact.contact_type = con_type.contact_type;
                    data_contact.contact_type_id = con_type.id
                }
                None => ()
            }

            contacts.push(data_contact);
        }

        Ok(contacts)
    }

    //---------------------------------------------------------------------------
    //get address by staff id
    //---------------------------------------------------------------------------
    async fn get_address_staff_id(
        &self,
        staff_id: Uuid) -> types::Response<Vec<Address>> {
        let staff_address = TblStaffAddress::find()
            .filter(tbl_staff_address::Column::StaffId.eq(staff_id.clone())
                .and(tbl_staff_address::Column::Primary.eq(true)))
            .all(self.conn.as_ref()).await?;

        let mut address_list: Vec<Address> = Vec::new();

        for staff_addr in staff_address {
            let addresses = TblAddress::find()
                .filter(tbl_address::Column::Id.eq(staff_addr.address_id.clone()))
                .one(self.conn.as_ref())
                .await?;

            let data_address: Address;

            match addresses {
                Some(addr) => {
                    data_address = Address {
                        id: addr.id,
                        street_name: addr.street_name,
                        suburb: addr.suburb,
                        post_code: addr.post_code,
                        state: addr.state,
                        country: addr.country,
                        primary: staff_addr.primary,
                    };

                    address_list.push(data_address)
                }
                None => continue
            }
        }

        Ok(address_list)
    }

    //----------------------------------------------------------------------------
    // get staff of given staff ids and store them against staff within the map.
    //----------------------------------------------------------------------------
    async fn get_staffs_ids(
        &self,
        ids: Vec<Uuid>) -> types::Response<Vec<Staff>> {
        let staffs_list = TblStaff::find()
            .filter(tbl_staff::Column::Id.is_in(ids.clone()))
            .all(self.conn.as_ref())
            .await?;

        let mut staff_list: Vec<data::Staff> = Vec::new();

        for staff in staffs_list {
            let hour_rate = staff.hourly_rate.to_f32().unwrap_or_else(|| 0.0);

            let mut data_staff = data::Staff {
                id: staff.id,
                first_name: staff.first_name,
                last_name: staff.last_name,
                email_address: staff.email_address,
                vehicle_registration: staff.vehicle_registration,
                staff_type_id: staff.staff_type_id,
                staff_type: "".to_string(),
                contractor_id: staff.contractor_id,
                sex: staff.sex,
                hourly_rate: hour_rate,
                active: staff.active,
                contacts: Vec::new(),
                address: Vec::new(),
            };

            let staff_type = TblStaffType::find()
                .filter(tbl_staff_type::Column::Id
                    .eq(staff.staff_type_id.clone())).one(self.conn.as_ref()).await?;

            match staff_type {
                Some(stf_type)=>{
                    data_staff.staff_type=stf_type.type_name
                }
                None=>()            }

            staff_list.push(data_staff);
        }

        Ok(staff_list)
    }

    //----------------------------------------------------------------------------
    // get contact of given staff ids and store them against staff within the map.
    //----------------------------------------------------------------------------
    async fn get_contacts_staff_ids(
        &self,
        staff_ids: Vec<Uuid>) -> types::Response<HashMap<String, Contact>> {
        let contacts_staff = TblStaffContact::find()
            .filter(tbl_staff_contact::Column::StaffId.is_in(staff_ids.clone())
                .and(tbl_staff_contact::Column::Primary.eq(true)))
            .all(self.conn.as_ref())
            .await?;

        let mut contact_map: HashMap<String, data::Contact> = HashMap::new();

        for stf_contact in contacts_staff {
            let contact = TblContact::find()
                .filter(tbl_contact::Column::Id.eq(stf_contact.contact_id.clone()))
                .one(self.conn.as_ref())
                .await?;

            let mut data_contact: Contact = Contact::default();
            match contact {
                Some(con) => {
                    data_contact.id = con.id;
                    data_contact.contact = con.contact_value
                }
                None => continue
            }

            let contact_type = TblContactType::find()
                .filter(tbl_contact_type::Column::Id.eq(stf_contact.contact_type_id.clone()))
                .one(self.conn.as_ref())
                .await?;

            match contact_type {
                Some(con_type) => {
                    data_contact.contact_type = con_type.contact_type;
                    data_contact.contact_type_id = con_type.id
                }
                None => ()
            }

            contact_map.insert(stf_contact.staff_id.to_string(), data_contact);
        }

        Ok(contact_map)
    }

    //----------------------------------------------------------------------------
    // get address of given staff ids and store them against staff within the map.
    //----------------------------------------------------------------------------
    async fn get_address_staff_ids(
        &self,
        staff_ids: Vec<Uuid>) -> types::Response<Vec<Address>> {
        let staff_address_list = TblStaffAddress::find()
            .filter(tbl_staff_address::Column::StaffId.is_in(staff_ids.clone())
                .and(tbl_staff_address::Column::Primary.eq(true)))
            .all(self.conn.as_ref()).await?;

        let mut address_list: Vec<Address> = Vec::new();

        for stf_add in staff_address_list {
            let addresses_list = TblAddress::find()
                .filter(tbl_address::Column::Id.eq(stf_add.address_id.clone()))
                .one(self.conn.as_ref())
                .await?;

            let data_address: Address;

            match addresses_list {
                Some(addr) => {
                    data_address = Address {
                        id: addr.id,
                        street_name: addr.street_name,
                        suburb: addr.suburb,
                        post_code: addr.post_code,
                        state: addr.state,
                        country: addr.country,
                        primary:stf_add.primary
                    };

                    address_list.push(data_address)
                }
                None => continue
            }
        }

        Ok(address_list)
    }
}
