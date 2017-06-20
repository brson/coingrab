pub struct Coin {
    pub sym: &'static str,
    pub name: &'static str,
}

lazy_static! {
    pub static ref COINS: Vec<Coin> = define_coins();
}

fn define_coins() -> Vec<Coin> {
    vec![
        Coin {
            sym: "1st",
            name: "FirstBlood",
        },
        Coin {
            sym: "ANS",
            name: "AntShares",
        },
        Coin {
            sym: "ARK",
            name: "Ark",
        },
        Coin {
            sym: "BAT",
            name: "Basic Attention Token",
        },
        Coin {
            sym: "BCC",
            name: "BitConnect",
        },
        Coin {
            sym: "BCN",
            name: "Bytecoin",
        },
        Coin {
            sym: "BTC",
            name: "Bitcoin",
        },
        Coin {
            sym: "BTS",
            name: "BitShares",
        },
        Coin {
            sym: "DASH",
            name: "Dash",
        },
        Coin {
            sym: "DCR",
            name: "Decred",
        },
        Coin {
            sym: "DGB",
            name: "DigiByte",
        },
        Coin {
            sym: "DOGE",
            name: "Dogecoin",
        },
        Coin {
            sym: "ESP",
            name: "Espers",
        },
        Coin {
            sym: "ETC",
            name: "Ethereum Classic",
        },
        Coin {
            sym: "ETH",
            name: "Ethereum",
        },
        Coin {
            sym: "FCT",
            name: "Factom",
        },
        Coin {
            sym: "GAME",
            name: "GameCredits",
        },
        Coin {
            sym: "GNO",
            name: "Gnosis",
        },
        Coin {
            sym: "GNT",
            name: "Golem",
        },
        Coin {
            sym: "LSK",
            name: "Lisk",
        },
        Coin {
            sym: "LTC",
            name: "Litecoin",
        },
        Coin {
            sym: "MAID",
            name: "MaidSafeCoin",
            // keywords: vec!["MaidSafe"]
        },
        Coin {
            sym: "NXT",
            name: "Nxt",
        },
        Coin {
            sym: "REP",
            name: "Augur",
        },
        Coin {
            sym: "PTOY",
            name: "Patientory",
        },
        Coin {
            sym: "SC",
            name: "Siacoin",
        },
        Coin {
            sym: "STEEM",
            name: "Steem",
        },
        Coin {
            sym: "STRAT",
            name: "Stratis",
        },
        Coin {
            sym: "WAVES",
            name: "Waves",
        },
        Coin {
            sym: "XEM",
            name: "NEM",
        },
        Coin {
            sym: "XLM",
            name: "Stellar Lumens",
            // keywords: vec!["Stellar"],
        },
        Coin {
            sym: "XMR",
            name: "Monero",
        },
        Coin {
            sym: "XRP",
            name: "Ripple",
        },
        Coin {
            sym: "ZEC",
            name: "Zcash",
        },
    ]
}
