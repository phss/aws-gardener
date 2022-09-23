use aws_gardener::aws::s3::S3;
use tokio::net::TcpSocket;

#[tokio::test]
async fn lists_buckets() {
    require_localstack().await;

    let s3 = S3::new_for_localstack().await;
    let buckets = s3.list_buckets().await.unwrap();

    let bucket_names: Vec<String> = buckets.into_iter().map(|bucket| bucket.name).collect();

    assert_eq!(bucket_names, vec!["some-bucket", "another-bucket"]);
}

async fn require_localstack() {
    let socket = TcpSocket::new_v4().unwrap();
    let connection = socket.connect("127.0.0.1:4566".parse().unwrap()).await;
    assert!(
        connection.is_ok(),
        "Integration tests require localstack. Please ensure you have it running locally (docker compose -f tests/docker-compose.yml up -d)."
    );
}
