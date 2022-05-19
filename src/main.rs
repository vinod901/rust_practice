// #![allow(unused)]
mod services;
use anyhow::{Ok, Result};
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::*;

use services::s3::*;

#[tokio::main]
async fn main() -> Result<()> {
    let client = get_aws_client().await?;
    let buckets = show_buckets(&client).await?;
    for bucket in &buckets {
        list_objects(&client, bucket).await?;
    }

    // create_bucket(&client, "new-bucket-by-vinod901", REGION).await?;

    // client
    //     .delete_bucket()
    //     .bucket("https://new-bucket-by-vinod901.s3.us-west-2.amazonaws.com/")
    //     .send()
    //     .await?;

    // copy_object(
    //     &client,
    //     "atom-sandbox",
    //     "test_files/adventureworks/DatabaseLog.csv",
    //     "test_files/adventureworks/NewDatabaseLog.csv",
    // )
    // .await?;

    // delete_object(
    //     &client,
    //     &buckets[5],
    //     "test_files/adventureworks/NewDatabaseLog.csv",
    // )
    // .await?;

    // download_object(
    //     &client,
    //     "atom-sandbox",
    //     "test_files/adventureworks/DatabaseLog.csv",
    // )
    // .await?;

    // list_objects(&client, "atom-sandbox").await?;

    upload_object(
        &client,
        "atom-sandbox",
        "src/main.rs",
        "test_files/src/main.rs",
    )
    .await?;
    Ok(())
}

async fn get_aws_client() -> Result<Client> {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);
    Ok(client)
}
