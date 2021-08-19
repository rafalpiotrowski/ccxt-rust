use ccxt::coinbase::Coinbase;
use ccxt::kraken::Kraken;
use ccxt::exchange::{ServerTime, SystemStatus};

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

        let status = kraken.get_status().await.unwrap();
        println!("Kraken is: {}", status);
    });

    let _t1 = match h.await {
        Ok(_) => println!("coinbase completed"),
        Err(e) => println!("coinbase failed: {:?}", e)
    };
    let _t2 = match h1.await {
        Ok(_) => println!("kraken completed"),
        Err(e) => println!("kraken failed: {:?}", e)
    };
}