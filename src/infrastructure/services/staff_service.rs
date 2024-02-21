use std::sync::Arc;
use async_trait::async_trait;
use uuid::Uuid;
use crate::adapters::repository::{
    IMutationRepository,
    ISelectionRepository};
use crate::adapters::service;
use crate::adapters::service::IStaffService;
use crate::adapters::transfer::{
    AddressData,
    ContactData,
    ContactTypeData,
    StaffData,
    StaffTypeData};
use crate::adapters::types::Response;

pub struct StaffService<S,M:>
    where S:ISelectionRepository,M:IMutationRepository {
 pub sel_repo:Box<Arc<S>>,
 pub mut_repo:Box<Arc<M>>
}

impl <S,M>StaffService<S, M> where S:ISelectionRepository, M:IMutationRepository {
    pub fn new(sel_repo:Box<Arc<S>> ,mut_repo:Box<Arc<M>>)->Self<>{
        return Self{
            sel_repo,
            mut_repo,
        }
    }
}

#[async_trait]
impl <S,M>IStaffService for StaffService<S,M>
    where S:ISelectionRepository+ Sync + Send,
          M:IMutationRepository+ Sync + Send{
    async fn get_staff_by_staff_id(&self, staff_id: Uuid) -> Response<StaffData> {
       let xx = &(**self.sel_repo);

       let staff=  xx.get_staff_by_id(staff_id).await?;
       let staff_data = StaffData::from(staff);

        

        todo!()
    }

    async fn get_staff_by_first_name(&self, staff_id: Uuid) -> Response<Vec<StaffData>> {
        todo!()
    }

    async fn get_address_by_staff_id(&self, staff_id: Uuid) -> Response<Vec<AddressData>> {
        todo!()
    }

    async fn get_contact_by_staff_id(&self, staff_id: Uuid) -> Response<Vec<ContactData>> {
        todo!()
    }

    async fn get_all_staff_types(&self) -> Response<Vec<StaffTypeData>> {
        todo!()
    }

    async fn get_all_contact_types(&self) -> Response<Vec<ContactTypeData>> {
        todo!()
    }

    async fn upsert_staff(&self, staff: StaffData) -> Response<bool> {
        todo!()
    }
}


