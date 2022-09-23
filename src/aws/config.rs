use std::str::FromStr;

use aws_config::{meta::region::RegionProviderChain, SdkConfig};
use aws_sdk_s3::Endpoint;
use http::Uri;

pub struct Config {
    pub sdk_config: SdkConfig,
}

impl Config {
    pub async fn from_env() -> Self {
        let region_provider = RegionProviderChain::default_provider().or_else("eu-west-1");
        let config = aws_config::from_env().region(region_provider).load().await;
        Config { sdk_config: config }
    }

    pub async fn for_localstack(localstack_address: &'static str) -> Self {
        let region_provider = RegionProviderChain::default_provider().or_else("eu-west-1");
        let config = aws_config::from_env()
            .endpoint_resolver(Endpoint::immutable(
                Uri::from_str(format!("http://{}", localstack_address).as_str()).unwrap(),
            ))
            .region(region_provider)
            .load()
            .await;
        Config { sdk_config: config }
    }
}
