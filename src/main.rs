#![allow(unused)]

use anyhow::{anyhow, bail, Context, Ok, Result};
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{
    model::{BucketLocationConstraint, CreateBucketConfiguration},
    types::ByteStream,
    *,
};
use std::{env, path::Path};

const ENV_CRED_KEY_ID: &str = "S3_ID";
const ENV_CRED_KEY_SECRET: &str = "S3_SECRET_KEY";
const BUCKET_NAME: &str = "atom-sandbox";
const REGION: &str = "us-west-2";

#[tokio::main]
async fn main() -> Result<()> {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
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

async fn show_buckets(client: &Client) -> Result<Vec<String>> {
    let resp = client.list_buckets().send().await?;
    let buckets: Vec<String> = resp
        .buckets()
        .unwrap_or_default()
        .iter()
        .map(|bucket| bucket.name().unwrap_or_default().to_string())
        .collect();
    Ok(buckets)
}

async fn copy_object(client: &Client, bucket: &str, source: &str, destination: &str) -> Result<()> {
    let destination_path = format!("{}/{}", &bucket, &source);
    client
        .copy_object()
        .copy_source(destination_path)
        .bucket(bucket)
        .key(destination)
        .send()
        .await?;
    Ok(())
}

async fn delete_object(client: &Client, bucket: &str, object: &str) -> Result<()> {
    client
        .delete_object()
        .bucket(bucket)
        .key(object)
        .send()
        .await?;
    Ok(())
}

async fn create_bucket(client: &Client, bucket: &str, region: &str) -> Result<()> {
    let constraint = BucketLocationConstraint::from(region);
    let config = CreateBucketConfiguration::builder()
        .location_constraint(constraint)
        .build();
    client
        .create_bucket()
        .create_bucket_configuration(config)
        .bucket(bucket)
        .send()
        .await?;
    Ok(())
}

async fn download_object(client: &Client, bucket: &str, key: &str) -> Result<()> {
    let resp = client.get_object().bucket(bucket).key(key).send().await?;
    let data = resp.body.collect().await?;
    println!(
        "Data from downloaded object: {:?}",
        data.into_bytes().slice(0..20)
    );
    Ok(())
}

async fn list_objects(client: &Client, bucket: &str) -> Result<()> {
    let objects = client.list_objects_v2().bucket(bucket).send().await?;
    for object in objects.contents().unwrap_or_default().iter() {
        println!("  {}", object.key().unwrap());
    }
    Ok(())
}

async fn upload_object(client: &Client, bucket: &str, file: &str, key: &str) -> Result<()> {
    let body = ByteStream::from_path(Path::new(file)).await?;
    client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(body)
        .send()
        .await?;
    Ok(())
}
