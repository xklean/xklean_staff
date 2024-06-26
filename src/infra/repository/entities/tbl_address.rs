//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(schema_name = "cls_staff", table_name = "tbl_address")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub street_name: String,
    pub suburb: String,
    pub post_code: String,
    pub state: String,
    pub country: String,
    pub tenant_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::tbl_staff_address::Entity")]
    TblStaffAddress,
}

impl Related<super::tbl_staff_address::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TblStaffAddress.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
