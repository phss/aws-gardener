use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::Client;

use super::errors::AwsError;

pub struct Bucket {
    pub name: String,
}

impl From<&aws_sdk_s3::model::Bucket> for Bucket {
    fn from(bucket: &aws_sdk_s3::model::Bucket) -> Self {
        Bucket {
            name: bucket.name().unwrap_or_default().to_string(),
        }
    }
}

pub struct S3 {
    client: Client,
}

impl S3 {
    pub async fn new() -> Self {
        let region_provider = RegionProviderChain::default_provider().or_else("eu-west-1");
        let config = aws_config::from_env().region(region_provider).load().await;
        let client = Client::new(&config);
        S3 { client }
    }

    pub async fn list_buckets(self) -> Result<Vec<Bucket>, AwsError> {
        let response = self.client.list_buckets().send().await?;
        let buckets = response.buckets().unwrap_or_default();

        let buckets = buckets.into_iter().map(Bucket::from).collect();
        Ok(buckets)
    }
}
