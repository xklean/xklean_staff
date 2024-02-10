use sea_orm::prelude::async_trait;
use sea_orm_migration::{MigrationName, MigrationTrait, MigratorTrait};

pub struct Migrator;

impl MigrationName for Migrator {
    fn name(&self) -> &str {
        return "StaffMigration"
    }
}

#[async_trait::async_trait]
impl MigratorTrait for Migrator{
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        todo!()
    }
}