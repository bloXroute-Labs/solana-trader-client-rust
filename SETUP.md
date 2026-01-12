# solana-trader-client-rust

# Environment setup
set the necessary environment variables:

```bash
export AUTH_HEADER=<YOUR_AUTH_HEADER>
export NETWORK=<TARGET_NETWORK>
export REGION=<TARGET_REGION>
```

**Available networks:**
- MAINNET
- MAINNET_PUMP
- TESTNET
- LOCAL

**Available regions:**
- NY
- UK

You can also save these constants to a .env file like so:

```bash
PUBLIC_KEY="...."
PRIVATE_KEY="......."
AUTH_HEADER="......"
NETWORK=MAINNET
REGION=NY
```

### Vscode 
Tests can also be ran/debugged on click with vscode. 
Just add a `settings.json` inside the `.vscode` folder, paste this snippet, and fill in the auth key:

```json
{
    "rust-analyzer.runnables.extraEnv": {
        "PRIVATE_KEY": "...",
        "PUBLIC_KEY": "...",
        "AUTH_HEADER": "...",
        "NETWORK": "MAINNET",
    },
    "rust-analyzer.runnables.extraArgs": [
        "--",
        "--ignored",
        "--nocapture"
    ],
}
```

**If no region is defined, the SDK will use default to LOCAL**

# Running tests

Since these tests are networked, they have the ignore flag on by default:

```rust
    #[tokio::test]
    #[ignore]
```

So each test must be called individually:

```bash
cargo test test_get_rate_limit_grpc -- --ignored 
```

If you want to see output from a given test, add the `nocapture` flag:

```bash
cargo test test_get_rate_limit_grpc -- --ignored --nocapture
```

run `cargo clippy --tests` after adding your tests, to resolve any potential issues 

## Adding new test cases
Using the `test_case` crate tests are parametrized:

```rust
// old test
#[test_case("BTC", "USDC", 0.1, 0.1 ; "BTC to USDC with 0.1% slippage")]
// new test case
#[test_case("BTC", "USDC", 0.1, 10 ; "BTC to USDC with 10% slippage")]
```