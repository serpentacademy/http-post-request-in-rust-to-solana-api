extern crate reqwest;
use std::error::Error;
use serde_json::json;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();

    let json_body = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getAccountInfo",
        "params": [
            "F65xgN7bUhaJffKhtFotQsZEgd8DLviWt59qBMA4LfC5",
            {
                "encoding": "base58"
            }
        ]
    
    });



let res = client
    .post("https://api.devnet.solana.com")
    .json(&json_body)
    .send()
    .await
    .expect("failed to get response")
    .text()
    .await
    .expect("failed to get payload");

    println!("{:?}", res);



    
    Ok(())
}

