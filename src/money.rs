use rusty_money::{Money, define_currency_set};


define_currency_set!(
  currency_type_usd {
    USD: {
      code: "USD",
      exponent: 2,
      locale: Locale::EnUs,
      minor_units: 100,
      name: "DOLLAR",
      symbol: "USD",
      symbol_first: true,
    }
  }
);

define_currency_set!(
  currency_type_eur {
    EUR: {
      code: "EUR",
      exponent: 2,
      locale: Locale::EnUs,
      minor_units: 100,
      name: "EURO",
      symbol: "EUR",
      symbol_first: true,
    }
  }
);


pub(crate) fn get_usd_data(amount: i64,name: &str) {

    let data = currency_type_usd::find(name).unwrap();
   let getter =  Money::from_major(amount, data);

    println!("Deposited: {:?} USD", getter.amount())
}


pub(crate) fn get_eur_data(amount: i64,name: &str) {

  let data = currency_type_eur::find(name).unwrap();
 let getter =  Money::from_major(amount, data);

  println!("Deposited {:?} EUR", getter.amount())
}



