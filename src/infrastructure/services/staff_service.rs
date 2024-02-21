use std::sync::Arc;
use async_trait::async_trait;
use uuid::Uuid;
use crate::adapters::repository::{
    IMutationRepository,
    ISelectionRepository};

use crate::adapters::service::IStaffService;
use crate::adapters::transfer::{
    AddressData,
    ContactData,
    ContactTypeData,
    StaffData,
    StaffTypeData};
use crate::adapters::types::Response;

pub struct StaffService<S, M:>
    where S: ISelectionRepository, M: IMutationRepository {
    pub sel_repo: Box<Arc<S>>,
    pub mut_repo: Box<Arc<M>>,
}

impl<S, M> StaffService<S, M> where S: ISelectionRepository, M: IMutationRepository {
    pub fn new(sel_repo: Box<Arc<S>>, mut_repo: Box<Arc<M>>) -> Self <> {
        return Self {
            sel_repo,
            mut_repo,
        };
    }
}

#[async_trait]
impl<S, M> IStaffService for StaffService<S, M>
    where S: ISelectionRepository + Sync + Send,
          M: IMutationRepository + Sync + Send {
    //------------------------------------------------------------------------
    // implementation for select staff by id and related address and contacts.
    //------------------------------------------------------------------------
    async fn get_staff_by_staff_id(&self, staff_id: Uuid) -> Response<StaffData> {
        let select_repo = &(**self.sel_repo);

        let staff = select_repo.get_staff_by_id(staff_id).await?;
        let mut staff_data = StaffData::from(staff);

        let staff_address = select_repo.get_address_staff_id(staff_id).await?;


        for add in staff_address {
            let address = AddressData::from(add);
            staff_data.address.push(address)
        }

        let staff_contacts = select_repo.get_contacts_staff_id(staff_id).await?;
        for con in staff_contacts {
            let contact = ContactData::from(con);
            staff_data.contacts.push(contact)
        }

        Ok(staff_data)
    }

    //-------------------------------------------------------------------------
    //get staff by first name with related address and contacts
    //-------------------------------------------------------------------------
    async fn get_staff_by_first_name(
        &self,
        staff_first_name: String) -> Response<Vec<StaffData>> {
        let select_repo = &(**self.sel_repo);

        let mut staff_result: Vec<StaffData> = Vec::new();

        let staff_list = select_repo.get_staff_by_name(staff_first_name).await?;

        for stf in staff_list {
            let mut staff = StaffData::from(stf);

            let staff_address = select_repo.get_address_staff_id(staff.id.clone()).await?;
            for add in staff_address {
                let address = AddressData::from(add);
                staff.address.push(address)
            }

            let staff_contacts = select_repo.get_contacts_staff_id(staff.id.clone()).await?;
            for con in staff_contacts {
                let contact = ContactData::from(con);
                staff.contacts.push(contact)
            }

            staff_result.push(staff)
        }

        Ok(staff_result)
    }

    //------------------------------------------------------------------------
    //get address by staff id
    //------------------------------------------------------------------------
    async fn get_address_by_staff_id(
        &self, staff_id: Uuid) -> Response<Vec<AddressData>> {
        let select_repo = &(**self.sel_repo);

        let mut address_result: Vec<AddressData> = Vec::new();

        let staff_address = select_repo.get_address_staff_id(staff_id).await?;

        for add in staff_address {
            let address = AddressData::from(add);

            address_result.push(address)
        }

        Ok(address_result)
    }

    //------------------------------------------------------------------------
    //get contacts by staff id
    //------------------------------------------------------------------------
    async fn get_contact_by_staff_id(
        &self, staff_id: Uuid) -> Response<Vec<ContactData>> {
        let select_repo = &(**self.sel_repo);

        let mut contact_result: Vec<ContactData> = Vec::new();

        let staff_contacts = select_repo.get_contacts_staff_id(staff_id.clone()).await?;
        for con in staff_contacts {
            let contact = ContactData::from(con);
            contact_result.push(contact)
        }

        Ok(contact_result)
    }

    //------------------------------------------------------------------------
    //get all staff types
    //------------------------------------------------------------------------
    async fn get_all_staff_types(
        &self) -> Response<Vec<StaffTypeData>> {
        let select_repo = &(**self.sel_repo);

        let mut staff_type_result: Vec<StaffTypeData> = Vec::new();

        let staff_types = select_repo.get_all_staff_types().await?;

        for stf in staff_types {
            let staff_type = StaffTypeData::from(stf);

            staff_type_result.push(staff_type)
        }

        Ok(staff_type_result)
    }

    //------------------------------------------------------------------------
    //get all contact types
    //------------------------------------------------------------------------

    async fn get_all_contact_types(
        &self) -> Response<Vec<ContactTypeData>> {
        let select_repo = &(**self.sel_repo);

        let contact_types =select_repo.get_all_contact_types().await?;
        let contact_type_result=  contact_types.into_iter().map(|con_type|{
            let val= ContactTypeData::from(con_type);
            return val

        }).collect();

        Ok(contact_type_result)
    }

    async fn upsert_staff(
        &self,
        staff: StaffData) -> Response<bool> {


     Ok(true)
    }
}


