use aws_sdk_s3::Client;

use super::{errors::AwsError, Config};

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
    pub async fn new(config: Config) -> Self {
        let client = aws_sdk_s3::Client::new(&config.sdk_config);
        S3 { client }
    }

    pub async fn list_buckets(self) -> Result<Vec<Bucket>, AwsError> {
        let response = self.client.list_buckets().send().await?;
        let buckets = response.buckets().unwrap_or_default();

        let buckets = buckets.into_iter().map(Bucket::from).collect();
        Ok(buckets)
    }
}

#[cfg(test)]
mod test {
    use crate::aws::s3::Bucket;

    #[test]
    fn bucket_from_s3_model_bucket() {
        let s3_bucket = aws_sdk_s3::model::Bucket::builder()
            .set_name(Some("some-bucket".into()))
            .build();
        assert_eq!(Bucket::from(&s3_bucket).name, "some-bucket");
    }

    #[test]
    fn bucket_from_s3_model_bucket_with_no_name() {
        let s3_bucket = aws_sdk_s3::model::Bucket::builder().set_name(None).build();
        assert_eq!(Bucket::from(&s3_bucket).name, "");
    }
}
