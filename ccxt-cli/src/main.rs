use ccxt::coinbase::Coinbase;
use ccxt::exchange::ServerTime;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let coinbase = Coinbase::new();
    let time = coinbase.get_time().await.unwrap();

    println!("Coinbase time: {}", time);
}