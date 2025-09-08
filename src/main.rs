use mc_client::{types::{Internal, Registry}, Client};

#[tokio::main]
async fn main(){
    let mut cli = Client::build()
    .with_host("localhost")
    .with_port(25565)
    .with_username("test");

    let mut entities = cli.entities();

    entities.on_move(|entity| {
        println!("Entity {} move! X: {}, Y: {}, Z: {}",
        entity.id.0,
        entity.x,
        entity.y,
        entity.z);
    });

    entities.on_spawn(|entity| {
        println!("Entity {} spawn!", entity.id.0);
    });

    entities.on_remove(|entity| {
        println!("Entities {} despawn!", entity.ids.data[0].0);
    });

    let mut player = cli.player();

    player.on_synchronize(|player| {
        println!("Im moved! X: {}, Y: {}, Z: {}",
        player.x,
        player.y,
        player.z)
    });

    let mut client = cli.connect().await.unwrap();

    client.internal().bootstrap();
    tokio::spawn(client.read());
}