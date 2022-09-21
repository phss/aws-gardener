use aws_sdk_s3::{error::ListBucketsError, types::SdkError, Error};

#[derive(Debug)]
pub struct AwsError(Error);

impl From<SdkError<ListBucketsError>> for AwsError {
    fn from(error: SdkError<ListBucketsError>) -> Self {
        AwsError(error.into())
    }
}
