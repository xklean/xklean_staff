//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(schema_name = "cls_staff", table_name = "tbl_staff_type")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub type_name: String,
    pub tenant_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::tbl_staff::Entity")]
    TblStaff,
}

impl Related<super::tbl_staff::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TblStaff.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}