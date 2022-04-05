extern crate reqwest;
use std::error::Error;
use serde_json::json;
use std::collections::HashMap;
use reqwest::StatusCode;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();

    let json_body = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getAccountInfo",
        "params": [
            "9m5kFDqgpf7Ckzbox91RYcADqcmvxW4MmuNvroD5H2r9",
            {
                "encoding": "base58"
            }
        ]
    
    });



let client = reqwest::Client::new();
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

