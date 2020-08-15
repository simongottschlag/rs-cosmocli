use anyhow::Result;
use azure_sdk_cosmos::prelude::*;
use log::*;

pub struct CosmosDbItem {
    db_id: String,
    container_id: String,
}

pub async fn get_all_items(
    cosmosdb_account_name: String,
    cosmosdb_account_key: String,
) -> Result<()> {
    let authorization_token = AuthorizationToken::new_master(&cosmosdb_account_key)?;
    let client = ClientBuilder::new(&cosmosdb_account_name, authorization_token)?;
    let databases = client.list_databases().execute().await?;
    //let mut database_ids: Vec<String> = vec![];
    for database in databases.databases {
        //database_ids.push(database.id);
        let collections = client
            .with_database_client(&database.id)
            .list_collections()
            .execute()
            .await?;
        for collection in collections.collections {
            let documents = client
            .with_database_client(&database.id).with_collection_client(&collection.id).list_documents().max_item_count().execute()
        }
    }
    Ok(())
}
