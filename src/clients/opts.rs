use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "CosmosDB CLI written in Rust")]
pub struct Opts {
    #[structopt(subcommand)]
    pub commands: Option<Commands>,
}

#[derive(StructOpt, Debug)]
pub enum Commands {
    #[structopt(name = "backup", about = "Take backup of CosmosDB")]
    Backup(BackupOpts),
}

#[derive(StructOpt, Debug)]
pub struct BackupOpts {
    #[structopt(flatten)]
    pub config: BackupConfig,
    #[structopt(subcommand)]
    pub commands: Option<BackupSubCommands>,
}

#[derive(StructOpt, Debug)]
pub struct BackupConfig {
    #[structopt(
        name = "cosmosdb-account-name",
        long = "cosmosdb-account-name",
        env = "COSMOCLI_COSMOSDB_ACCOUNT_NAME"
    )]
    pub cosmosdb_account_name: String,
    #[structopt(
        name = "cosmosdb-account-key",
        long = "cosmosdb-account-key",
        env = "COSMOCLI_COSMOSDB_ACCOUNT_KEY"
    )]
    pub cosmosdb_account_key: String,
}

#[derive(StructOpt, Debug)]
pub enum BackupSubCommands {
    AzureStorageAccount(BackupAzureStorageAccount),
    Filesystem(BackupFilesystem),
}

#[derive(StructOpt, Debug)]
#[structopt(about = "Store backups in Azure Storage Account")]
pub struct BackupAzureStorageAccount {
    #[structopt(
        name = "storage-account-name",
        long = "storage-account-name",
        env = "COSMOCLI_STORAGE_ACCOUNT_NAME"
    )]
    storage_account_name: String,
    #[structopt(
        name = "storage-account-container",
        long = "storage-account-container",
        env = "COSMOCLI_STORAGE_ACCOUNT_CONTAINER"
    )]
    storage_account_container: String,
    #[structopt(
        name = "storage-account-key",
        long = "storage-account-key",
        env = "COSMOCLI_STORAGE_ACCOUNT_KEY"
    )]
    storage_account_key: String,
}

#[derive(StructOpt, Debug)]
#[structopt(about = "Store backups in Azure Storage Account")]
pub struct BackupFilesystem {
    #[structopt(
        name = "filesystem-path",
        long = "filesystem-path",
        env = "COSMOCLI_FILESYSTEM_PATH"
    )]
    filesystem_path: String,
}
