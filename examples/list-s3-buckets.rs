use aws_gardener::aws::{errors::AwsError, Config, S3};

#[tokio::main]
async fn main() -> Result<(), AwsError> {
    let config = Config::from_env().await;
    let s3 = S3::new(config).await;
    let buckets = s3.list_buckets().await?;

    println!("Buckets:");
    for bucket in &buckets {
        println!("  {}", bucket.name);
    }
    println!();
    println!("Found {} buckets", buckets.len());

    Ok(())
}
