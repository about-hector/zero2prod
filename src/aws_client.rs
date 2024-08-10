use aws_config::BehaviorVersion;
use aws_sdk_sesv2 as sesv2;

pub async fn create_aws_client() -> sesv2::Client {
   // let region_provider = RegionProviderChain::default_provider().or_else("eu-central-1");

    let config = aws_config::defaults(BehaviorVersion::latest())
        //.region(region_provider)
        .profile_name("admin")
        .load()
        .await;

    let client = sesv2::Client::new(&config);
    client
}
