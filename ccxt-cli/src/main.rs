use ccxt::coinbase::Coinbase;
use ccxt::kraken::Kraken;
use ccxt::exchange::{ApiConfig, Balance, ServerTime, SystemStatus};
use std::fs::File;
use std::io::Read;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let path = std::env::current_dir().unwrap();
    println!("The current directory is {}", path.display());

    let h = tokio::spawn(async {
        println!("Coinbase get time");
        let coinbase = Coinbase::new("coinbase");
        let time = coinbase.get_time().await.unwrap();
        println!("Coinbase time: {}", time);
        });

    let h1 = tokio::spawn(async {
        let file = File::open("api_kraken.json")
            .unwrap();
        let api_config: ApiConfig = serde_json::from_reader(file)
            .unwrap();

        println!("Kraken get time");
        let kraken = Kraken::new("kraken", api_config.key, api_config.secret);

        let time = kraken.get_time().await.unwrap();
        println!("Kraken time: {}", time);

        let status = kraken.get_status().await.unwrap();
        println!("Kraken is: {}", status);

        let b = kraken.get_balance().await;
        match b {
            Ok(a) => println!("Kraken balance is: {:?}", a),
            Err(x) => println!("Kraken balance failed: {:?}", x) 
        };

        let token = kraken.get_web_socket_token().await;
        match token {
            Ok(t) => println!("Kraken web socket token: {:?}", t),
            Err(x) => println!("Kraken web token failed: {:?}", x) 
        }
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