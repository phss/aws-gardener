use aws_gardener::aws::s3::S3;

#[tokio::test]
async fn lists_buckets() {
    let s3 = S3::new_for_localstack().await;
    let buckets = s3.list_buckets().await.unwrap();

    let bucket_names: Vec<String> = buckets.into_iter().map(|bucket| bucket.name).collect();

    assert_eq!(bucket_names, vec!["some-bucket", "another-bucket"]);
}
