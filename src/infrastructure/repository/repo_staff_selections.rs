use std::collections::HashMap;
use std::sync::{Arc};
use async_trait::async_trait;
use sea_orm::{DatabaseConnection, entity::*, query::*};
use uuid::Uuid;
use crate::adapters::repository::ISelectionRepository;
use entities::{prelude::*,
               tbl_staff,
               tbl_contact,
               tbl_contact_type,
               tbl_staff_address,
               tbl_address};
use crate::adapters::{entities as ent};
use crate::adapters::entities::{AddressEntity};
use crate::adapters::errors::ServiceErr;
use crate::infrastructure::repository::entities::{tbl_staff_contact, tbl_staff_type};
use crate::adapters::types;
use crate::infrastructure::repository::entities::prelude::TblStaff;


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
        tenant_id:Uuid,
        id: Uuid) -> types::Response<StaffEntity> {
        let staff = TblStaff::find()
            .filter(tbl_staff::Column::Id.eq(id.clone())
                .and(tbl_staff::Column::TenantId.eq(tenant_id.clone())))
            .one(self.conn.as_ref())
            .await?;

        let data_staff = match staff {
            Some(stf) => {
                let mut data_result = StaffEntity::from(stf);

                let staff_type = TblStaffType::find()
                    .filter(tbl_staff_type::Column::Id
                        .eq(data_result.staff_type_id.clone())
                        .and(tbl_staff_type::Column::TenantId.eq(tenant_id.clone()))).one(self.conn.as_ref()).await?;

                match staff_type {
                    Some(stf_type) => {
                        data_result.staff_type = stf_type.type_name
                    }
                    None => ()
                }

                Ok(data_result)
            }
            None => {
                Err(ServiceErr("error in entities".to_string()))
            }
        };

        return data_staff;
    }

    //---------------------------------------------------------------------------
    //get contact by staff id
    //---------------------------------------------------------------------------
    async fn get_contacts_staff_id(
        &self,
        tenant_id:Uuid,
        staff_id: Uuid) -> types::Response<Vec<ent::ContactEntity>> {
        let contacts_staff = TblStaffContact::find()
            .filter(tbl_staff_contact::Column::StaffId.eq(staff_id.clone())
                .and(tbl_staff_contact::Column::TenantId.eq(tenant_id.clone())))
            .all(self.conn.as_ref())
            .await?;

        let mut contacts: Vec<ent::ContactEntity> = Vec::new();

        for stf_contact in contacts_staff {
            let contact = TblContact::find()
                .filter(tbl_contact::Column::Id.eq(stf_contact.contact_id.clone())
                    .and(tbl_contact::Column::TenantId.eq(tenant_id.clone())))
                .one(self.conn.as_ref())
                .await?;

            let mut data_contact: ent::ContactEntity = ent::ContactEntity::default();
            match contact {
                Some(con) => {
                    data_contact.id = con.id;
                    data_contact.contact = con.contact_value
                }
                None => continue
            }

            let contact_type = TblContactType::find()
                .filter(tbl_contact_type::Column::Id.eq(stf_contact.contact_type_id.clone())
                    .and(tbl_contact_type::Column::TenantId.eq(tenant_id.clone())))
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
        tenant_id:Uuid,
        staff_id: Uuid) -> types::Response<Vec<AddressEntity>> {
        let staff_address = TblStaffAddress::find()
            .filter(tbl_staff_address::Column::StaffId.eq(staff_id.clone())
                .and(tbl_staff_address::Column::TenantId.eq(tenant_id.clone()))
                .and(tbl_staff_address::Column::Primary.eq(true)))
            .all(self.conn.as_ref()).await?;

        let mut address_list: Vec<AddressEntity> = Vec::new();

        for staff_addr in staff_address {
            let addresses = TblAddress::find()
                .filter(tbl_address::Column::Id.eq(staff_addr.address_id.clone())
                    .and(tbl_address::Column::TenantId.eq(tenant_id.clone())))
                .one(self.conn.as_ref())
                .await?;

            let data_address: AddressEntity;

            match addresses {
                Some(addr) => {
                    data_address = AddressEntity {
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
        tenant_id:Uuid,
        ids: Vec<Uuid>) -> types::Response<Vec<StaffEntity>> {
        let staffs_list = TblStaff::find()
            .filter(tbl_staff::Column::Id.is_in(ids.clone())
                .and(tbl_staff::Column::TenantId.eq(tenant_id.clone())))
            .all(self.conn.as_ref())
            .await?;

        let mut staff_list: Vec<ent::StaffEntity> = Vec::new();

        for staff in staffs_list {
            let mut data_staff = StaffEntity::from(staff);

            let staff_type = TblStaffType::find()
                .filter(tbl_staff_type::Column::Id
                    .eq(data_staff.staff_type_id.clone())
                    .and(tbl_staff_type::Column::TenantId.eq(tenant_id.clone())))
                .one(self.conn.as_ref()).await?;

            match staff_type {
                Some(stf_type) => {
                    data_staff.staff_type = stf_type.type_name
                }
                None => ()
            }

            staff_list.push(data_staff);
        }

        Ok(staff_list)
    }

    //----------------------------------------------------------------------------
    // get contact of given staff ids and store them against staff within the map.
    //----------------------------------------------------------------------------
    async fn get_contacts_staff_ids(
        &self,
        tenant_id:Uuid,
        staff_ids: Vec<Uuid>) -> types::Response<HashMap<String, ent::ContactEntity>> {
        let contacts_staff = TblStaffContact::find()
            .filter(tbl_staff_contact::Column::StaffId.is_in(staff_ids.clone())
                .and(tbl_staff_contact::Column::TenantId.eq(tenant_id.clone()))
                .and(tbl_staff_contact::Column::Primary.eq(true)))
            .all(self.conn.as_ref())
            .await?;

        let mut contact_map: HashMap<String, ent::ContactEntity> = HashMap::new();

        for stf_contact in contacts_staff {
            let contact = TblContact::find()
                .filter(tbl_contact::Column::Id.eq(stf_contact.contact_id.clone())
                    .and(tbl_contact::Column::TenantId.eq(tenant_id.clone())))
                .one(self.conn.as_ref())
                .await?;

            let mut data_contact: ent::ContactEntity = ent::ContactEntity::default();
            match contact {
                Some(con) => {
                    data_contact.id = con.id;
                    data_contact.contact = con.contact_value
                }
                None => continue
            }

            let contact_type = TblContactType::find()
                .filter(tbl_contact_type::Column::Id.eq(stf_contact.contact_type_id.clone())
                    .and(tbl_contact_type::Column::TenantId.eq(tenant_id.clone())))
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
        tenant_id:Uuid,
        staff_ids: Vec<Uuid>) -> types::Response<Vec<AddressEntity>> {
        let staff_address_list = TblStaffAddress::find()
            .filter(tbl_staff_address::Column::StaffId.is_in(staff_ids.clone())
                .and(tbl_staff_address::Column::TenantId.eq(tenant_id.clone()))
                .and(tbl_staff_address::Column::Primary.eq(true)))
            .all(self.conn.as_ref()).await?;

        let mut address_list: Vec<AddressEntity> = Vec::new();

        for stf_add in staff_address_list {
            let addresses_list = TblAddress::find()
                .filter(tbl_address::Column::Id.eq(stf_add.address_id.clone())
                    .and(tbl_address::Column::TenantId.eq(tenant_id.clone())))
                .one(self.conn.as_ref())
                .await?;

            let data_address: AddressEntity;

            match addresses_list {
                Some(addr) => {
                    data_address = AddressEntity {
                        id: addr.id,
                        street_name: addr.street_name,
                        suburb: addr.suburb,
                        post_code: addr.post_code,
                        state: addr.state,
                        country: addr.country,
                        primary: stf_add.primary,
                    };

                    address_list.push(data_address)
                }
                None => continue
            }
        }

        Ok(address_list)
    }

    //----------------------------------------------------------------------------
    // get staff by first name.
    //----------------------------------------------------------------------------
    async fn get_staff_by_name(
        &self,
        tenant_id:Uuid,
        name: String) -> Response<Vec<StaffEntity>> {
        let staff_list = TblStaff::find()
            .filter(tbl_staff::Column::FirstName.like(name.clone())
                .and(tbl_staff::Column::TenantId.eq(tenant_id.clone())))
            .all(self.conn.as_ref())
            .await?;

        let mut staff_data_list: Vec<ent::StaffEntity> = Vec::new();

        for stf in staff_list {
            let mut data_staff = StaffEntity::from(stf);

            let staff_type = TblStaffType::find()
                .filter(tbl_staff_type::Column::Id
                    .eq(data_staff.staff_type_id.clone())
                    .and(tbl_staff_type::Column::TenantId.eq(tenant_id.clone())))
                .one(self.conn.as_ref()).await?;

            match staff_type {
                Some(stf_type) => {
                    data_staff.staff_type = stf_type.type_name
                }
                None => ()
            }

            staff_data_list.push(data_staff);
        }

        Ok(staff_data_list)
    }

    //----------------------------------------------------------------------------
    // get all staff types.
    //----------------------------------------------------------------------------
    async fn get_all_staff_types(&self,tenant_id:Uuid) -> Response<Vec<StaffTypeEntity>> {
        let mut staff_type_result: Vec<StaffTypeEntity> = Vec::new();

        let staff_types = TblStaffType::find()
            .filter(tbl_staff_type::Column::TenantId.eq(tenant_id.clone()))
            .all(self.conn.as_ref()).await?;

        for sft in staff_types {
            let staff_type = StaffTypeEntity::from(sft);

            staff_type_result.push(staff_type)
        }

        Ok(staff_type_result)
    }

    //----------------------------------------------------------------------------
    // get all contact types.
    //----------------------------------------------------------------------------
    async fn get_all_contact_types(&self,tenant_id:Uuid) -> Response<Vec<ContactTypeEntity>> {
        let mut contact_type_result: Vec<ContactTypeEntity> = Vec::new();

        let contact_types = TblContactType::find()
            .filter(tbl_contact_type::Column::TenantId.eq(tenant_id.clone()))
            .all(self.conn.as_ref()).await?;

        for con in contact_types {
            let con_type = ContactTypeEntity::from(con);

            contact_type_result.push(con_type)
        }

        Ok(contact_type_result)
    }
}
