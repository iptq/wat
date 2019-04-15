use packer::Packer;

#[derive(Packer)]
#[packer(source = "migrations/postgres")]
pub struct PostgresMigrations;

#[derive(Packer)]
#[packer(source = "migrations/mysql")]
pub struct MysqlMigrations;

#[derive(Packer)]
#[packer(source = "migrations/sqlite")]
pub struct SqliteMigrations;
