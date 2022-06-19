use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::{Client,meta::AttributeValue};
use lambda_runtime::{handler_fn,Context,Error as LambdaError};
use serde::{Deserialize,Serialize};
use serde_json::{ json, Value };
use uuid::Uuid;

#[derive(Deserialize,Serialize)]
struct CustomEvent{
    first_name: String,
    last_name:String,
}

#[tokio::main]
async fn main()->Result<(),LambdaError>{
    let func=handler_fn(handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn handler(event:CustomEvent,_:Context)->Result<Value,LambdaError>{
    Uuid::new_v4().to_string();
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config=aws_config::from_env().region(region_provider);
    let client=Client::new(&config);
    let request = client.put_item().table_name("user").item("uid",AttributeValue::S(String::from(uuid))).item("first_name",AttributeValue::S(String::from(event.first_name))).item("last_name",AttributeValue::S(String::from(event.last_name)));
    request.send().await()?;
    Ok(json!({"message" : "Record inserted successfully"}))
}