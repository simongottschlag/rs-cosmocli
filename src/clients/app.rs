use crate::clients;
use anyhow::Result;
use log::*;
use structopt::StructOpt;

pub async fn app() -> Result<()> {
    let opt = clients::opts::Opts::from_args();
    if let Some(app) = opt.commands {
        match app {
            clients::opts::Commands::Backup(command) => backup(command).await?,
        }
    }
    Ok(())
}

async fn backup(command: clients::opts::BackupOpts) -> Result<()> {
    let backup_config = command.config;
    debug!("CosmosDB config: {:?}", backup_config);
    clients::cosmosdb::get_all_items(
        backup_config.cosmosdb_account_name,
        backup_config.cosmosdb_account_key,
    )
    .await?;
    if let Some(sub_command) = command.commands {
        match sub_command {
            clients::opts::BackupSubCommands::AzureStorageAccount(storageaccount_config) => {
                debug!("Azure Storage Account Config: {:?}", storageaccount_config)
            }
            clients::opts::BackupSubCommands::Filesystem(filesystem_config) => {
                debug!("Filesystem Config: {:?}", filesystem_config)
            }
        }
    }
    Ok(())
}
