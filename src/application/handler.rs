use std::sync::Arc;
use config::Config;
use tonic::{Code, Request, Response, Status};
use crate::adapters::repository::IMutationRepository;
use crate::adapters::service::IStaffService;
use crate::config::configuration::Configuration;
use crate::pb_staff;
use crate::pb_staff::{RequestAddressUpsert, RequestContactTypes, RequestContactTypeUpsert, RequestContactUpsert, RequestStaffById, RequestStaffFirstName, RequestStaffTypes, RequestStaffTypeUpsert, RequestStaffUpsert, ResponseAddressByStaffId, ResponseAddressUpsert, ResponseContactsByStaffId, ResponseContactTypes, ResponseContactTypeUpsert, ResponseContactUpsert, ResponseStaffByFirstName, ResponseStaffById, ResponseStaffTypes, ResponseStaffTypeUpsert, ResponseStaffUpsert};
use crate::pb_staff::staff_service_server::{StaffService};

use uuid::Error as UuidError;


pub struct StaffServiceApi {
    pub config: Box<Arc<Configuration>>,
    pub staff_service: Box<dyn IStaffService>,
}

impl StaffServiceApi {
    pub fn new(
        cfg: Box<Arc<Configuration>>, staff_service: Box<dyn IStaffService>) -> Self {
        return StaffServiceApi {
            config: cfg,
            staff_service,
        };
    }
}

#[tonic::async_trait]
impl StaffService for StaffServiceApi {
    async fn get_staff_by_staff_id(
        &self,
        request: Request<RequestStaffById>) -> Result<Response<ResponseStaffById>, Status> {
        let request_data = request.into_inner();

        if request_data.id == String::from("") {
            return Err(Status::new(
                Code::InvalidArgument,
                "staff_id is not valid"));
        }

        if request_data.tenant_id == String::from("") {
            return Err(Status::new(
                Code::InvalidArgument,
                "tenant_id is not valid"));
        }

        let id = Uuid::parse_str(
            request_data.id.as_str());

        let id = match id {
            Ok(id) => id,
            Err(_) => return Err(Status::new(
                Code::InvalidArgument,
                "staff_id is not valid")),
        };


        let tenant_id = Uuid::parse_str(
            request_data.tenant_id.as_str());

        let tenant_id = match tenant_id {
            Ok(id) => id,
            Err(_) => return Err(Status::new(
                Code::InvalidArgument,
                "tenant_id is not valid")),
        };


        let staff_result = self.staff_service
            .get_staff_by_staff_id(tenant_id.clone(), id).await;

        return match staff_result {
            Ok(staff_data) => {
                let staff_data = pb_staff::Staff::from(staff_data);

                let response = pb_staff::ResponseStaffById {
                    staff: Some(staff_data),
                };

                Ok(Response::new(response))
            }
            Err(_) => Err(Status::new(
                Code::Internal,
                "Failed to get staff by staff id")),
        };
    }


    async fn get_staff_by_first_name(
        &self,
        request: Request<RequestStaffFirstName>) -> Result<Response<ResponseStaffByFirstName>, Status> {
        let request_data = request.into_inner();

        if request_data.tenant_id == String::from("") {
            return Err(Status::new(
                Code::InvalidArgument,
                "tenant_id is not valid"));
        }

        if request_data.first_name.is_empty() {
            return Err(Status::new(
                Code::InvalidArgument,
                "first_name is not defined"));
        }


        let tenant_id = Uuid::parse_str(
            request_data.tenant_id.as_str());

        let tenant_id = match tenant_id {
            Ok(id) => id,
            Err(_) => return Err(Status::new(
                Code::InvalidArgument,
                "tenant_id is not valid")),
        };


        let staff_result = self.staff_service
            .get_staff_by_first_name(
                tenant_id,
                request_data.first_name).await;

        return match staff_result {
            Ok(result) => {
                let stf_result = result.into_iter()
                    .map(|staff_data| -> pb_staff::Staff{
                    let staff_data = pb_staff::Staff::from(staff_data);
                    return staff_data;
                }).collect();

                let response = pb_staff::ResponseStaffByFirstName {
                    staff: stf_result,
                };

                Ok(Response::new(response))
            }
            Err(_) => return Err(Status::new(
                Code::Internal,
                "Failed to get staff data")),
        };
    }

    async fn get_address_by_staff_id(
        &self,
        request: Request<RequestStaffById>) -> Result<Response<ResponseAddressByStaffId>, Status> {
        let request_data = request.into_inner();

        let staff_id = Uuid::parse_str(
            request_data.id.as_str());

        let staff_id = match staff_id {
            Ok(id) => id,
            Err(_) => return Err(Status::new(
                Code::InvalidArgument,
                "staff_id is not valid")),
        };

        let tenant_id = Uuid::parse_str(
            request_data.tenant_id.as_str());

        let tenant_id = match tenant_id {
            Ok(id) => id,
            Err(_) => return Err(Status::new(
                Code::InvalidArgument,
                "tenant_id is not valid")),
        };


        let address = self.staff_service.
            get_address_by_staff_id(tenant_id, staff_id).await;

        return match address {
            Ok(add_data) => {
                let add_result = add_data.into_iter()
                    .map(|add| -> pb_staff::Address{
                        return pb_staff::Address::from(add);
                    }).collect();

                let response = pb_staff::ResponseAddressByStaffId {
                    address: add_result,
                };

                Ok(Response::new(response))
            }
            Err(_) => {
                return Err(Status::new(Code::InvalidArgument, "address is not found"));
            }
        };
    }

    async fn get_contacts_by_staff_id(
        &self,
        request: Request<RequestStaffById>) -> Result<Response<ResponseContactsByStaffId>, Status> {
        let request_data = request.into_inner();

        let staff_id = Uuid::parse_str(
            request_data.id.as_str());

        let staff_id = match staff_id {
            Ok(id) => id,
            Err(_) => return Err(
                Status::new(Code::InvalidArgument,
                            "staff id is not defined"))
        };

        let tenant_id = Uuid::parse_str(
            request_data.tenant_id.as_str());


        let tenant_id = match tenant_id {
            Ok(id) => id,
            Err(_) => return Err(Status::new(
                Code::InvalidArgument,
                "tenant_id is not valid")),
        };


        let contacts = self
            .staff_service.get_contact_by_staff_id(tenant_id, staff_id).await;

        return match contacts {
            Ok(contact_data) => {
                let con_result = contact_data
                    .into_iter().map(|cont| -> pb_staff::Contact{
                    return pb_staff::Contact::from(cont);
                }).collect();

                let response = pb_staff::ResponseContactsByStaffId {
                    contacts: con_result
                };

                Ok(Response::new(response))
            }
            Err(_) => {
                return Err(Status::new(Code::Internal, "error in selection"));
            }
        };
    }

    async fn get_all_staff_type(
        &self,
        request: Request<RequestStaffTypes>) -> Result<Response<ResponseStaffTypes>, Status> {
        let request_data = request.into_inner();

        let tenant_id = Uuid::parse_str(
            request_data.tenant_id.as_str());


        let tenant_id = match tenant_id {
            Ok(id) => id,
            Err(_) => return Err(Status::new(
                Code::InvalidArgument,
                "tenant_id is not valid")),
        };

        let staff_types = self
            .staff_service.get_all_staff_types(tenant_id).await;


        return match staff_types {
            Ok(staff_type_data) => {
                let staff_type_result = staff_type_data
                    .into_iter().map(|types_data| -> pb_staff::StaffType{
                    return pb_staff::StaffType::from(types_data);
                }).collect();

                Ok(Response::new(ResponseStaffTypes {
                    staff_types: staff_type_result,
                }))
            }
            Err(_) => {
                return Err(Status::new(Code::Internal, "error selection staff types"));
            }
        };
    }

    async fn get_all_contact_type(
        &self,
        request: Request<RequestContactTypes>) -> Result<Response<ResponseContactTypes>, Status> {
        let request_data = request.into_inner();

        let tenant_id = Uuid::parse_str(
            request_data.tenant_id.as_str());


        let tenant_id = match tenant_id {
            Ok(id) => id,
            Err(_) => return Err(Status::new(
                Code::InvalidArgument,
                "tenant_id is not valid")),
        };

        let contact_types = self
            .staff_service.get_all_contact_types(tenant_id).await;


        return match contact_types {
            Ok(contact_type_data) => {
                let contact_type_result = contact_type_data
                    .into_iter().map(|types_data| -> pb_staff::ContactType{
                    return pb_staff::ContactType::from(types_data);
                }).collect();

                Ok(Response::new(ResponseContactTypes {
                    contact_types: contact_type_result,
                }))
            }
            Err(_) => {
                return Err(Status::new(Code::Internal, "error selection staff types"));
            }
        };
    }

    async fn upsert_staff(
        &self,
        request: Request<RequestStaffUpsert>) -> Result<Response<ResponseStaffUpsert>, Status> {
        let request_data = request.into_inner();

        let tenant_id = Uuid::parse_str(
            request_data.tenant_id.as_str());

        let tenant_id = match tenant_id {
            Ok(id) => id,
            Err(_) => return Err(Status::new(
                Code::InvalidArgument,
                "tenant_id is not valid")),
        };

       let staff_data = match request_data.staff{
            Some(st)=> {
                 StaffData::from(st)
            }

            None => return Err(
                Status::new(Code::Internal,
                            "error update staff types"))
        };

        let staff_result =  self.staff_service.upsert_staff(tenant_id,staff_data).await;
        match staff_result{
            Ok(result)=> {
                Ok(Response::new(ResponseStaffUpsert{
                    success:result
                }))
            }
            Err(_) => {
                Err(Status::new(Code::Internal,
                            "error update staff "))
            }
        }
    }

    async fn upsert_address(
        &self,
        request: Request<RequestAddressUpsert>) -> Result<Response<ResponseAddressUpsert>, Status> {
        let request_data = request.into_inner();

        let tenant_id = Uuid::parse_str(
            request_data.tenant_id.as_str());

        let tenant_id = match tenant_id {
            Ok(id) => id,
            Err(_) => return Err(Status::new(
                Code::InvalidArgument,
                "tenant_id is not valid")),
        };

        let staff_id = Uuid::parse_str(
            request_data.staff_id.as_str());

        let staff_id = match staff_id {
            Ok(id) => id,
            Err(_) => return Err(
                Status::new(Code::InvalidArgument,
                            "staff id is not defined"))
        };


        let address_data = match request_data.address{
            Some(add)=> {
                AddressData::from(add)
            }

            None => return Err(Status::new(Code::Internal,"error update address"))
        };

        let address_result = self.staff_service
            .upsert_address(tenant_id,staff_id,address_data).await;

        match address_result {
            Ok(result)=>{
                Ok(Response::new(ResponseAddressUpsert {
                    success:result
                }))
            }
             Err(_)=> {
                 return Err(Status::new(Code::Internal,
                                        "error update address "))
            }
        }
    }

    async fn upsert_contact(
        &self,
        request: Request<RequestContactUpsert>) -> Result<Response<ResponseContactUpsert>, Status> {
        let request_data = request.into_inner();

        let tenant_id = Uuid::parse_str(
            request_data.tenant_id.as_str());

        let tenant_id = match tenant_id {
            Ok(id) => id,
            Err(_) => return Err(Status::new(
                Code::InvalidArgument,
                "tenant_id is not valid")),
        };

        let staff_id = Uuid::parse_str(
            request_data.staff_id.as_str());

        let staff_id = match staff_id {
            Ok(id) => id,
            Err(_) => return Err(
                Status::new(Code::InvalidArgument,
                            "staff id is not defined"))
        };

        let contact_data = match request_data.contact{
            Some(con)=> {
                ContactData::from(con)
            }

            None => return Err(
                Status::new(
                    Code::Internal,"error update contact"))
        };

        let contact_result = self.staff_service
            .upsert_contact(tenant_id,staff_id,contact_data).await;

        match contact_result {
            Ok(result)=>{
                Ok(Response::new(ResponseContactUpsert {
                    success:result
                }))
            }
            Err(_)=> {
                return Err(Status::new(Code::Internal,
                                       "error update contact "))
            }
        }
     }

    async fn upsert_staff_type(
        &self,
        request: Request<RequestStaffTypeUpsert>) -> Result<Response<ResponseStaffTypeUpsert>, Status> {
        let request_data = request.into_inner();

        let tenant_id = Uuid::parse_str(
            request_data.tenant_id.as_str());

        let tenant_id = match tenant_id {
            Ok(id) => id,
            Err(_) => return Err(Status::new(
                Code::InvalidArgument,
                "tenant_id is not valid")),
        };


        let staff_type_data = match request_data.staff_type{
            Some(staff_type)=> {
                StaffTypeData::from(staff_type)
            }

            None => return Err(
                Status::new(
                    Code::Internal,"error update staff type"))
        };

        let staff_type_result = self.staff_service
            .upsert_staff_type(tenant_id,staff_type_data).await;

        match staff_type_result {
            Ok(result)=>{
                Ok(Response::new(ResponseStaffTypeUpsert {
                    success:result
                }))
            }
            Err(_)=> {
                return Err(Status::new(Code::Internal,
                                       "error update contact "))
            }
        }
    }

    async fn upsert_contact_type(
        &self,
        request: Request<RequestContactTypeUpsert>) -> Result<Response<ResponseContactTypeUpsert>, Status> {
        let request_data = request.into_inner();

        let tenant_id = Uuid::parse_str(
            request_data.tenant_id.as_str());

        let tenant_id = match tenant_id {
            Ok(id) => id,
            Err(_) => return Err(Status::new(
                Code::InvalidArgument,
                "tenant_id is not valid")),
        };


        let contact_type_data = match request_data.contact_type{
            Some(contact_type)=> {
                ContactTypeData::from(contact_type)
            }

            None => return Err(
                Status::new(
                    Code::Internal,"error update contact type"))
        };

        let contact_type_result = self.staff_service
            .upsert_contact_type(tenant_id,contact_type_data).await;

        match contact_type_result {
            Ok(result)=>{
                Ok(Response::new(ResponseContactTypeUpsert {
                    success:result
                }))
            }
            Err(_)=> {
                return Err(Status::new(Code::Internal,
                                       "error update contact type "))
            }
        }
    }
}

