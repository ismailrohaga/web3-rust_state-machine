use std::collections::BTreeMap;

pub struct Pallet {
	block_number: u32,
	nonce: BTreeMap<String, u32>,
}

impl Pallet {
	pub fn new() -> Self {
		Self { block_number: 0, nonce: BTreeMap::new() }
	}

	pub fn get_block_number(&self) -> u32 {
		self.block_number
	}

	pub fn inc_block_number(&mut self) {
		self.block_number = self.block_number.checked_add(1).unwrap();
	}

	pub fn get_nonce(&self, account: &str) -> u32 {
		*self.nonce.get(account).unwrap_or(&0)
		// self.nonce.get(account).unwrap_or(&0).clone() this approach will take a copy of the value
	}

	pub fn inc_nonce(&mut self, account: &str) {
		let nonce = self.nonce.get(account).unwrap_or(&0);
		self.nonce.insert(account.to_string(), nonce.checked_add(1).unwrap());
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn init_system() {
		let system = super::Pallet::new();
		assert_eq!(system.get_block_number(), 0);
	}

	#[test]
	fn inc_block_number() {
		let mut system = super::Pallet::new();
		system.inc_block_number();
		assert_eq!(system.get_block_number(), 1);
	}

	#[test]
	fn inc_nonce() {
		let mut system = super::Pallet::new();
		system.inc_nonce("Alice");
		assert_eq!(system.get_nonce("Alice"), 1);
	}
}
