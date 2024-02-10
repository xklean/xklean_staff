use tonic::{Request, Response, Status};
use crate::pb_staff;
use crate::pb_staff::{RequestStaffById, ResponseStaffById};
use crate::pb_staff::staff_service_server::{StaffService};

pub struct StaffServiceApi();


impl StaffServiceApi{
    pub fn new()->Self{
        return StaffServiceApi()
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