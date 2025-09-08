use mc_client::{types::{Internal, Registry}, Client};

#[tokio::main]
async fn main(){
    let mut cli = Client::build()
    .with_host("localhost")
    .with_port(25565)
    .with_username("test");
    let mut client = cli.connect().await.unwrap();
    client.internal().bootstrap();

    client.read().await;
}