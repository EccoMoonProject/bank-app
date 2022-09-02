use hex_literal::hex;
use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256, Sha512};
use std::fmt;
use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate)struct TxHash {
    pub(crate)amount: u128,
    pub(crate)hash: String,
}


pub(crate) fn encode_params(amount: u128, key: &[u8], value: &[u8]) {
    // Create alias for HMAC-SHA256
    type HmacSha256 = Hmac<Sha256>;

    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC can take key of any size");
    mac.update(value);

    // `result` has type `CtOutput` which is a thin wrapper around array of
    // bytes for providing constant time equality check
    let result = mac.finalize();
    // To get underlying array use `into_bytes`, but be careful, since
    // incorrect use of the code value may permit timing attacks which defeats
    // the security provided by the `CtOutput`
    let code_bytes = result.into_bytes();

    println!("{:x}", code_bytes);

    let x = format!("{:x}", code_bytes);

    let new_tx = crate::auth::TxHash {
        amount: amount.to_owned(),
        hash: x.to_owned(),
    };

    let  serialized = serde_json::to_string(&new_tx).unwrap();

    fs::write("./src/data/txhash.json", serialized);
}
