use serde::{Deserialize, Serialize};
use std::fs;
use std::io::stdin;
use std::{thread, time};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Wallet {
    pub(crate) id: String,
    pub(crate) deposit: u128,
    pub(crate) currency: String,
}

pub(crate) fn add_wallet_data(idData: &String, depositData: &u128, currencyData: &String) {
    let user1 = crate::wallet::Wallet {
        id: idData.to_owned(),
        deposit: depositData.to_owned(),
        currency: currencyData.to_owned(),
    };

    let serialized = serde_json::to_string(&user1).unwrap();

    fs::write("src/data/wallet.json", serialized);
}
pub(crate) fn create_user_wallet() {
    let ten_millis = time::Duration::from_millis(100);

    println!("Type your id data: ");
    let mut input_id = String::new();
    stdin()
        .read_line(&mut input_id)
        .ok()
        .expect("Failed to read line");

    println!("What amount is your deposit: ");
    let mut input_deposit = String::new();
    stdin()
        .read_line(&mut input_deposit)
        .ok()
        .expect("Failed to read line");

    let trimmed = input_deposit
        .trim()
        .parse::<u128>()
        .expect("wrong type: should be u128");

    // println!("{:?}", trimmed);

    println!("What is yours currency: ");
    let mut input_currency = String::new();
    stdin()
        .read_line(&mut input_currency)
        .ok()
        .expect("Failed to read line");

    let user_Wallet = add_wallet_data(&input_id, &trimmed, &input_currency);
}
