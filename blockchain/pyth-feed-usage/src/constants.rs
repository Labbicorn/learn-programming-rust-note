pub struct PriceFeedsConfig {
    pub name: String,
    pub key: String,
}

// feed ids https://pyth.network/developers/price-feed-ids#solana-mainnet-beta
impl PriceFeedsConfig {
    pub fn new(name: String, key: String) -> Self {
        Self { name, key }
    }

    pub fn default_price_feeds() -> Vec<PriceFeedsConfig> {
        vec![
            Self::new(
                "ETH/USD".to_string(),
                "JBu1AL4obBcCMqKBBxhpWCNUt136ijcuMZLFvTP7iWdB".to_string(),
            ),
            Self::new(
                "SOL/USD".to_string(),
                "H6ARHf6YXhGYeQfUzQNGk6rDNnLBQKrenN712K4AQJEG".into(),
            ),
            Self::new(
                "BTC/USD".to_string(),
                "GVXRSBjFk6e6J3NbVPXohDJetcTjaeeuykUpbQF8UoMU".into(),
            ),
            Self::new(
                "FTT/USD".to_string(),
                "8JPJJkmDScpcNmBRKGZuPuG2GYAveQgP3t5gFuMymwvF".into(),
            ),
            Self::new(
                "ATOM/USD".into(),
                "CrCpTerNqtZvqLcKqz1k13oVeXV9WkMD2zA9hBKXrsbN".to_string(),
            ),
            Self::new(
                "ATAS/USD".into(),
                "81Rz3i7MC9nHYo1vQg6kJM5hepjqb63Y1gnr3AkrD36D".to_string(),
            ),
            Self::new(
                "ARB/USD".into(),
                "5HRrdmghsnU3i2u5StaKaydS7eq3vnKVKwXMzCNKsc4C".to_string(),
            ),
            Self::new(
                "BONK/USD".into(),
                "8ihFLu5FimgTQ1Unh4dVyEHUGodJ5gJQCrQf4KUVB9bN".to_string(),
            ),
            Self::new(
                "GMT/USD".into(),
                "DZYZkJcFJThN9nZy4nK3hrHra1LaWeiyoZ9SMdLFEFpY".to_string(),
            ),
            Self::new(
                "HNT/USD".to_string(),
                "7moA1i5vQUpfDwSpK6Pw9s56ahB7WFGidtbL2ujWrVvm".to_string(),
            ),
            Self::new(
                "MNDE/USD".to_string(),
                "4dusJxxxiYrMTLGYS6cCAyu3gPn2xXLBjS7orMToZHi1".to_string(),
            ),
            Self::new(
                "ORCA/USD".to_string(),
                "4ivThkX8uRxBpHsdWSqyXYihzKF3zpRGAUCqyuagnLoV".to_string(),
            ),
            Self::new(
                "RAY/USD".to_string(),
                "AnLf8tVYCM816gmBjiy8n53eXKKEDydT5piYjjQDPgTB".to_string(),
            ),
            Self::new(
                "RLB/USD".to_string(),
                "4BA3RcS4zE32WWgp49vvvre2t6nXY1W1kMyKZxeeuUey".to_string(),
            ),
            Self::new(
                "RNDR/USD".to_string(),
                "CYGfrBJB9HgLf9iZyN4aH5HvUAi2htQ4MjPxeXMf4Egn".to_string(),
            ),
            Self::new(
                "RUNE/USD".to_string(),
                "7T1CEv5TXeheCiJeoXY7MwgeDH4YGEkVXkF3gwQP8EGD".to_string(),
            ),
        ]
    }
}
