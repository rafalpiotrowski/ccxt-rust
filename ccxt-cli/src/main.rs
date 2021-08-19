use ccxt::coinbase::Coinbase;
use ccxt::kraken::Kraken;
use ccxt::exchange::ServerTime;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let h = tokio::spawn(async {
        println!("Coinbase get time");
        let coinbase = Coinbase::new("coinbase");
        let time = coinbase.get_time().await.unwrap();
        println!("Coinbase time: {}", time);
        });

    let h1 = tokio::spawn(async {
        println!("Kraken get time");
        let kraken = Kraken::new("kraken");
        let time = kraken.get_time().await.unwrap();
        println!("Kraken time: {}", time);
    });

    let _t1 = h.await.unwrap();
    let _t2 = h1.await.unwrap();
}