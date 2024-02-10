use std::sync::Arc;
use config::Config;
use tonic::{Request, Response, Status};
use crate::config::configuration::Configuration;
use crate::pb_staff;
use crate::pb_staff::{RequestStaffById, ResponseStaffById};
use crate::pb_staff::staff_service_server::{StaffService};

#[derive(Default,Clone)]
pub struct StaffServiceApi{
    config:Box<Arc<Configuration>>
}


impl StaffServiceApi{
    pub fn new(cfg :Box<Arc<Configuration>>)->Self{
        return StaffServiceApi{
            config:cfg
        }
    }
}

impl StaffService for StaffServiceApi {
    async fn get_staff(&self, request: Request<RequestStaffById>) -> Result<Response<ResponseStaffById>, Status> {
        let req = request.get_ref();

        Ok(Response::new(ResponseStaffById{
            created_at:None
        }))
    }
}