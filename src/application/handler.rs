use std::sync::Arc;
use config::Config;
use tonic::{Code, Request, Response, Status};
use crate::adapters::repository::IMutationRepository;
use crate::adapters::service::IStaffService;
use crate::config::configuration::Configuration;
use crate::pb_staff;
use crate::pb_staff::{
    RequestContactTypes,
    RequestStaffById,
    RequestStaffFirstName,
    RequestStaffTypes,
    RequestStaffUpsert,
    ResponseAddressByStaffId,
    ResponseContactsByStaffId,
    ResponseContactTypes,
    ResponseStaffByFirstName,
    ResponseStaffById,
    ResponseStaffTypes,
    ResponseStaffUpsert};
use crate::pb_staff::staff_service_server::{StaffService};


pub struct StaffServiceApi{
    pub config:Box<Arc<Configuration>>,
    pub staff_service:Box<dyn IStaffService>

}

impl StaffServiceApi{
    pub fn new(
        cfg :Box<Arc<Configuration>>,staff_service:Box<dyn IStaffService>) ->Self{
        return StaffServiceApi{
            config:cfg,
            staff_service

        }
    }
}

#[tonic::async_trait]
impl StaffService for StaffServiceApi {
    async fn get_staff_by_staff_id(
        &self,
        request: Request<RequestStaffById>) -> Result<Response<ResponseStaffById>, Status> {
        let request_data = request.into_inner();

        if request_data.id == String::from(""){
           return Err(Status::new(Code::InvalidArgument, "staff_id is not valid"))
        }

        if request_data.tenant_id == String::from(""){
            return Err(Status::new(Code::InvalidArgument, "tenant_id is not valid"))
        }

        let id = Uuid::parse_str(request_data.id.as_str())?;
        let tenant_id = Uuid::parse_str(request_data.tenant_id.as_str())?;


        let staff_result =  self.staff_service.get_staff_by_staff_id(tenant_id.clone(),id).await?;

        todo!()
    }

    async fn get_staff_by_first_name(
        &self,
        request: Request<RequestStaffFirstName>) -> Result<Response<ResponseStaffByFirstName>, Status> {
        todo!()
    }

    async fn get_address_by_staff_id(
        &self,
        request: Request<RequestStaffById>) -> Result<Response<ResponseAddressByStaffId>, Status> {
        todo!()
    }

    async fn get_contacts_by_staff_id(
        &self,
        request: Request<RequestStaffById>) -> Result<Response<ResponseContactsByStaffId>, Status> {
        todo!()
    }

    async fn get_all_staff_type(
        &self,
        request: Request<RequestStaffTypes>) -> Result<Response<ResponseStaffTypes>, Status> {
        todo!()
    }

    async fn get_all_contact_type(
        &self,
        request: Request<RequestContactTypes>) -> Result<Response<ResponseContactTypes>, Status> {
        todo!()
    }

    async fn upsert_staff(
        &self,
        request: Request<RequestStaffUpsert>) -> Result<Response<ResponseStaffUpsert>, Status> {
        todo!()
    }
}