use async_trait::async_trait;
use sea_orm::{DatabaseConnection,entity::*, query::*};
use sea_orm::sea_query::all;
use uuid::Uuid;
use crate::adapters::repository::IRepository;
use entities::{prelude::*, tbl_staff};
use crate::infrastructure::repository::entities;
use crate::adapters::{data};
use crate::adapters::errors::ServiceErr;
use crate::infrastructure::repository::entities::tbl_staff_contact;

#[derive(Default)]
pub struct Repository {
    pub conn: DatabaseConnection,
}

impl Repository {
    pub fn new(db_conn: DatabaseConnection) -> Repository {
        return Repository {
            conn: db_conn
        };
    }
}

#[async_trait]
impl IRepository for Repository {
    async fn get_staff_by_id(&self, id: Uuid) -> Result<data::Staff, ServiceErr> {
        let staff = TblStaff::find()
            .filter(tbl_staff::Column::Id.eq(id.clone()))
            .one(&self.conn)
            .await?;

        let data_staff =  match staff {
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

        return data_staff
    }

    async fn get_contacts_staff_id(&self, staff_id: Uuid) -> Result<Vec<data::Contact>, ServiceErr> {
        let contacts_staff = TblStaffContact::find()
            .filter(tbl_staff_contact::Column::StaffId.eq(&staff_id)).all(self.conn.as_ref())
            .await?;

        for modl in contacts_staff{

        }

    }
}
