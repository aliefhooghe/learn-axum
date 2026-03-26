use sea_orm_migration::cli;
mod migrations;
mod migrator;

#[tokio::main]
async fn main() {
    cli::run_cli(migrator::Migrator).await;
}
