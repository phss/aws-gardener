use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{Client, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let region_provider = RegionProviderChain::default_provider().or_else("eu-west-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    let response = client.list_buckets().send().await?;
    let buckets = response.buckets().unwrap();

    println!("Buckets:");
    for thing in buckets {
        println!("  {}", thing.name().unwrap());
    }
    println!();
    println!("Found {} buckets", buckets.len());

    Ok(())
}
