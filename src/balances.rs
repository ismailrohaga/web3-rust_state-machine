use std::collections::BTreeMap;

pub struct Pallet {
	pub balances: BTreeMap<String, u128>,
}

impl Pallet {
	// Create a new pallet instance. Perhapas like a constructor.
	pub fn new() -> Self {
		Self { balances: BTreeMap::new() }
	}

	pub fn set_balance(&mut self, account: &str, balance: u128) {
		self.balances.insert(account.to_string(), balance);

		// the implementation can also expect a param of account: &String
		// and then when inserting, simply pass account.clone() to the map
		// explaination:
	}

	pub fn get_balance(&self, account: &str) -> u128 {
		*self.balances.get(account).unwrap_or(&0)
	}
}
