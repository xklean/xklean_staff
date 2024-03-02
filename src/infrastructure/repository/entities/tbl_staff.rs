//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(schema_name = "cls_staff", table_name = "tbl_staff")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email_address: String,
    pub vehicle_registration: Option<String>,
    pub staff_type_id: Uuid,
    pub tenant_id: Uuid,
    pub sex: String,
    pub active: bool,
    #[sea_orm(column_type = "Decimal(Some((7, 2)))")]
    pub hourly_rate: Decimal,
    pub created_at: DateTime,
    pub updated_at: Option<DateTime>,
    pub commence_date: Option<Date>,
    pub deleted_at: Option<DateTime>,
    pub operation_user_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::tbl_staff_address::Entity")]
    TblStaffAddress,
    #[sea_orm(has_many = "super::tbl_staff_contact::Entity")]
    TblStaffContact,
    #[sea_orm(
        belongs_to = "super::tbl_staff_type::Entity",
        from = "Column::StaffTypeId",
        to = "super::tbl_staff_type::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    TblStaffType,
}

impl Related<super::tbl_staff_address::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TblStaffAddress.def()
    }
}

impl Related<super::tbl_staff_contact::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TblStaffContact.def()
    }
}

impl Related<super::tbl_staff_type::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TblStaffType.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
