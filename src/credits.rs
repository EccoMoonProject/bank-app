use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Credits {
    pub name: String,
    pub amount: u128,
}

pub(crate) fn mint_credits(amount: u128) {

    let user1 = crate::credits::Credits {
        name: "credits".to_owned(),
        amount: amount * 1000,
    };

    let serialized = serde_json::to_string(&user1).unwrap();

    fs::write("src/data/credits.json", serialized);

}

