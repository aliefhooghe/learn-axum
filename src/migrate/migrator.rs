use crate::migrations;
use sea_orm_migration::{MigratorTrait, async_trait};

use migrations::m01_create_user;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn sea_orm_migration::MigrationTrait>> {
        vec![Box::new(m01_create_user::Migration)]
    }
}
