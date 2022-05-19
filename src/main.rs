mod services;
use anyhow::{Ok, Result};

use services::{dynamodb::*, s3::*};

#[tokio::main]
async fn main() -> Result<()> {
    let client = get_s3_client().await?;
    let buckets = list_buckets(&client).await?;
    println!("S3 buckets : {}", buckets.len());
    for bucket in &buckets {
        println!("  {}", bucket);
    }
    let dynamo_client = get_dynamo_client().await?;
    let tables = list_tables(&dynamo_client).await?;
    println!("\nDynamoDB tables : {}", tables.len());
    for table in tables {
        let items = list_items(&dynamo_client, table.as_str()).await?;
        println!("  {} : {} items", table, items.len());
        // for item in items {
        //     println!("      {:?}", item);
        // }
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

    // upload_object(
    //     &client,
    //     "atom-sandbox",
    //     "src/main.rs",
    //     "test_files/src/main.rs",
    // )
    // .await?;
    Ok(())
}
