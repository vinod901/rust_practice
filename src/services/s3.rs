use anyhow::{Ok, Result};
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{
    model::{BucketLocationConstraint, CreateBucketConfiguration},
    types::ByteStream,
    *,
};
use std::path::Path;

pub async fn get_s3_client() -> Result<Client> {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);
    Ok(client)
}

pub async fn list_buckets(client: &Client) -> Result<Vec<String>> {
    let resp = client.list_buckets().send().await?;
    let buckets: Vec<String> = resp
        .buckets()
        .unwrap_or_default()
        .iter()
        .map(|bucket| bucket.name().unwrap_or_default().to_string())
        .collect();
    Ok(buckets)
}

pub async fn copy_object(
    client: &Client,
    bucket: &str,
    source: &str,
    destination: &str,
) -> Result<()> {
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

pub async fn delete_object(client: &Client, bucket: &str, object: &str) -> Result<()> {
    client
        .delete_object()
        .bucket(bucket)
        .key(object)
        .send()
        .await?;
    Ok(())
}

pub async fn create_bucket(client: &Client, bucket: &str, region: &str) -> Result<()> {
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

pub async fn download_object(client: &Client, bucket: &str, key: &str) -> Result<()> {
    let resp = client.get_object().bucket(bucket).key(key).send().await?;
    let data = resp.body.collect().await?;
    println!(
        "Data from downloaded object: {:?}",
        data.into_bytes().slice(0..20)
    );
    Ok(())
}

pub async fn list_objects(client: &Client, bucket: &str) -> Result<()> {
    let objects = client.list_objects_v2().bucket(bucket).send().await?;
    for object in objects.contents().unwrap_or_default().iter() {
        println!("  {}", object.key().unwrap());
    }
    Ok(())
}

pub async fn upload_object(client: &Client, bucket: &str, file: &str, key: &str) -> Result<()> {
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
