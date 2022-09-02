use crate::auth::TxHash;
use crate::credits::Credits;
use crate::wallet::Wallet;
use rand::Rng;
use std::io::stdin;
use std::{fs, hash};
use std::{thread, time};
mod auth;
mod money;
mod wallet;
mod credits;
// deposit usd and change wallet depo status
fn deposit_usd(iso_standard: &str) {
    // user input
    println!("Deposit amount: ");
    let mut input_amount = String::new();
    stdin()
        .read_line(&mut input_amount)
        .ok()
        .expect("Failed to read line");

    // trim input to i64
    let amount = input_amount
        .trim()
        .parse::<i64>()
        .expect("wrong type: should be i64");

    // create money deposit instance
    let deposit_amount = crate::money::get_usd_data(amount, iso_standard);

    // read wallet struct and create new "p"
    let path = "./src/data/wallet.json";
    let data = fs::read_to_string(path).expect("Unable to read file");
    let mut p: Wallet = serde_json::from_str(&data).expect("Unable to");

    // serde_json...
    let serialized = serde_json::to_string(&amount).unwrap();
    let serialized_slice: &str = &serialized[..];
    let updated_amount: u128 = serialized_slice.trim().parse().unwrap();

    // update depo status
    p.deposit = p.deposit + updated_amount;

    // write file
    let updated_wallet = serde_json::to_string(&p).unwrap();
    fs::write(path, updated_wallet);

    // RNG to generate random number type i32
    let mut rng = rand::thread_rng();
    let x = rng.gen::<i32>();
    let val = x.to_string();
    let xc = val.as_bytes();

    let bytes_deposit = p.deposit.to_string();

    let xe = bytes_deposit.as_bytes();

    // encode unique params to every tx
    crate::auth::encode_params(updated_amount, &xc, xe);

    crate::credits::mint_credits(updated_amount);

    println!("{:?}", deposit_amount);
}

fn deposit_eur(iso_standard: &str) {
    // user input
    println!("Deposit amount: ");
    let mut input_amount = String::new();
    stdin()
        .read_line(&mut input_amount)
        .ok()
        .expect("Failed to read line");

    // trim input to i64
    let amount = input_amount
        .trim()
        .parse::<i64>()
        .expect("wrong type: should be i64");

    // create money deposit instance

    let deposit_amount = crate::money::get_eur_data(amount, iso_standard);

    // read wallet struct and create new "p"
    let path = "./src/data/wallet.json";
    let data = fs::read_to_string(path).expect("Unable to read file");
    let mut p: Wallet = serde_json::from_str(&data).expect("Unable to");

    // serde_json...
    let serialized = serde_json::to_string(&amount).unwrap();
    let serialized_slice: &str = &serialized[..];
    let updated_amount: u128 = serialized_slice.trim().parse().unwrap();

    // update depo status
    p.deposit = updated_amount;

    // write file
    let updated_wallet = serde_json::to_string(&p).unwrap();
    fs::write("./src/data/wallet.json", updated_wallet);

    println!("{:?}", deposit_amount);
}

// simple withdraw method -- to change...
// can withdraw only amount with proper tx hash
fn withdraw() {
    // read wallet struct and create new "p"
    let path = "./src/data/txhash.json";
    let data = fs::read_to_string(path).expect("Unable to read file");
    let mut struct_hash: TxHash = serde_json::from_str(&data).expect("Unable to");

       // read wallet struct and create new "p"
       let path_credits = "./src/data/credits.json";
       let data_cred = fs::read_to_string(path_credits).expect("Unable to read file");
       let mut struct_credits: Credits = serde_json::from_str(&data_cred).expect("Unable to");

    let tx_hash = struct_hash.hash;
    // user input
    println!("Put your secret tx_hash: ");
    let mut hash_input = String::new();
    stdin()
        .read_line(&mut hash_input)
        .ok()
        .expect("Failed to read line");

    if hash_input.trim() == tx_hash {
        // user input
        println!("Withdraw amount: ");
        let mut input_amount = String::new();
        stdin()
            .read_line(&mut input_amount)
            .ok()
            .expect("Failed to read line");

        // trim input to i64
        let amount = input_amount
            .trim()
            .parse::<i64>()
            .expect("wrong type: should be i64");

        let path = "./src/data/wallet.json";
        let data = fs::read_to_string(path).expect("Unable to read file");
        let mut p: Wallet = serde_json::from_str(&data).expect("Unable to");

        // serde_json...
        let serialized = serde_json::to_string(&amount).unwrap();
        let serialized_slice: &str = &serialized[..];
        let updated_amount: u128 = serialized_slice.trim().parse().unwrap();

        p.deposit = p.deposit - updated_amount;

        struct_credits.amount = struct_credits.amount - (updated_amount * 1000);

        if updated_amount <= struct_hash.amount {
            let updated_wallet = serde_json::to_string(&p).unwrap();
            let updated_credits = serde_json::to_string(&struct_credits).unwrap();
            fs::write(path, updated_wallet);
            fs::write(path_credits, updated_credits);
        } else {
            println!(
                "You cant withdraw more thank tx_hash amount: {}",
                updated_amount
            );
        }
    } else {
        println!("wrong hash - cannot withdraw...")
    }
}

fn main() {
    crate::wallet::create_user_wallet();

    // // DEPOSIT EUR
    // deposit_eur("EUR");

    // DEPOSIT USD
    deposit_usd("USD");

    // WITHDRAW METHOD

    withdraw();
}
