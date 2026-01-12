# Solana Trader Rust SDK

## Objective

This SDK is designed to make it easy for you to use the bloXroute Labs API in Rust. 

## Installation

``cargo add solana-trader-client-rust``

or

```toml
[dependencies]
solana-trader-client-rust = "0.1.1"
```

## Usage

The SDK provides access to Solana Trader API through:

- gRPC: High-performance RPC calls
- HTTP: Simple REST requests
- WebSocket: Real-time streaming data


### Client Initialization

Refer to **SETUP.md** for available networks, regions IDE setup and notes on testing.

Create and populate your `.env` file with something like this:

```bash
PUBLIC_KEY="...."
PRIVATE_KEY="......."
AUTH_HEADER="......"
NETWORK=MAINNET
REGION=NY
```

Please refer to the `tests` directory for more examples.

## Known issues and important notes
1. When running more than one integration test, you must use the flag `--test-threads=1`.
1. Using the network `TESTNET`, as detailed in `SETUP.md`, will submit the transaction to Solana mainnet. The `TESTNET` network setting will route your transaction to Solana Trader API's test instance which, in turn, will submit the transaction Solana mainnet.
1. The SDK is currently not multi-process safe. For example, the following code will panic:
```
use solana_trader_client_rust::{
    common::constants::SAMPLE_OWNER_ADDR,
    provider::grpc::GrpcClient,
};
use tokio::task;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let mut handles = vec![];
    for i in 0..5 {
        handles.push(task::spawn(async move {
            println!("Starting task {}", i);
            let mut client = GrpcClient::new(None).await.unwrap();

            match client.get_token_accounts(SAMPLE_OWNER_ADDR.to_string()).await {
                Ok(response) => {
                    println!(
                        "token accounts: {:?}",
                        serde_json::to_string_pretty(&response)
                    );
                }
                Err(e) => {
                    eprintln!("Failed to get token accounts: {:?}", e);
                }
            }       

            println!("Finished task {}", i);
        }));
    }

    for handle in handles {
        let _ = handle.await;
    }
}
```
