use std::collections::HashMap;

use anyhow::{Ok, Result};
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::{model::AttributeValue, Client};
use tokio_stream::StreamExt;

pub async fn get_dynamo_client() -> Result<Client> {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);
    Ok(client)
}

pub async fn list_tables(client: &Client) -> Result<Vec<String>> {
    let paginator = client.list_tables().into_paginator().items().send();
    let table_names: Vec<String> = paginator.collect::<Result<Vec<_>, _>>().await?;
    Ok(table_names)
}

pub async fn list_items(
    client: &Client,
    table: &str,
) -> Result<Vec<HashMap<String, AttributeValue>>> {
    let items_result: Result<Vec<_>, _> = client
        .scan()
        .table_name(table)
        .into_paginator()
        .items()
        .send()
        .collect()
        .await;
    let items = items_result?;
    Ok(items)
}
