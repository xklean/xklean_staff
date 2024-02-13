use std::sync::{Arc};
use async_trait::async_trait;
use sea_orm::{DatabaseConnection, entity::*, query::*};
use uuid::Uuid;
use data::Contact;
use crate::adapters::repository::IRepository;
use entities::{prelude::*,
               tbl_staff,
               tbl_contact,
               tbl_contact_type,
               tbl_staff_address,
               tbl_address};
use crate::infrastructure::repository::entities;
use crate::adapters::{data};
use crate::adapters::data::{Address};
use crate::adapters::errors::ServiceErr;
use crate::infrastructure::repository::entities::tbl_staff_contact;

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
impl IRepository for Repository {
    async fn get_staff_by_id(&self, id: Uuid) -> Result<data::Staff, ServiceErr> {
        let staff = TblStaff::find()
            .filter(tbl_staff::Column::Id.eq(id.clone()))
            .one(self.conn.as_ref())
            .await?;

        let data_staff = match staff {
            Some(stf) => {
                let data_result = data::Staff {
                    id,
                    first_name: stf.first_name,
                    last_name: stf.last_name,
                    email_address: stf.email_address,
                    vehicle_registration: stf.vehicle_registration,
                    staff_type_id: stf.staff_type_id,
                    contractor_id: stf.contractor_id,
                    sex: stf.sex,
                    contacts: vec![],
                    address: vec![],
                };

                Ok(data_result)
            }
            None => {
                Err(ServiceErr("error in data".to_string()))
            }
        };

        return data_staff;
    }

    async fn get_contacts_staff_id(&self, staff_id: Uuid) -> Result<Vec<Contact>, ServiceErr> {
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

    async fn get_address_staff_id(&self, staff_id: Uuid) -> Result<Vec<Address>, ServiceErr> {
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
                        suburb: addr.suberb,
                        post_code: addr.post_code,
                        state: addr.state,
                        country: addr.country,
                    };

                    address_list.push(data_address)
                }
                None => continue
            }
        }

        Ok(address_list)
    }
}
