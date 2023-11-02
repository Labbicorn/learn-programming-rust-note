// example usage of reading pyth price from solana/pythnet price account

use pyth_sdk_solana::load_price_feed_from_account;
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{thread, time};

pub mod constants;

fn main() -> anyhow::Result<()> {
    let url = "http:/pythnet.rpcpool.com";
    // Pyth ETH/USD price account on pythnet (can be found on https://pyth.network/developers/price-feed-ids#solana-mainnet-beta which has the same price feed IDs as pythnet)

    let clnt = RpcClient::new(url.to_string());
    let keys = constants::PriceFeedsConfig::default_price_feeds();
    let price_keys_with_name = keys
        .into_iter()
        .map(|e| (e.name, Pubkey::from_str(&e.key).unwrap()))
        .collect::<Vec<_>>();

    loop {
        for (name, key) in price_keys_with_name.clone() {
            // get price data from key
            let mut price_accounts = clnt.get_account(&key).unwrap();
            let price_feed = load_price_feed_from_account(&key, &mut price_accounts).unwrap();

            println!(".....{name}.....");

            let current_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64;

            let maybe_price = price_feed.get_price_no_older_than(current_time, 60);
            match maybe_price {
                Some(p) => {
                    println!(
                        "{name}-price ------------- {}",
                        (p.price as f64) * 10f64.powi(p.expo)
                    );

                    println!(
                        "{name}-conf ------------- {}",
                        (p.conf as f64) * 10f64.powi(p.expo)
                    );
                }
                None => {
                    println!("{name}price ........... unavailable");
                    println!("{name}conf ............ unavailable");
                }
            }
            println!();
            thread::sleep(time::Duration::from_secs(5));
        }
    }
}
