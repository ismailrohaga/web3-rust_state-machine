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

	pub fn transfer(
		&mut self,
		sender: &str,
		receiver: &str,
		amount: u128,
	) -> Result<(), &'static str> {
		let sender_balance = self.get_balance(sender);
		let receiver_balance = self.get_balance(receiver);

		let new_sender_balance =
			sender_balance.checked_sub(amount).ok_or("Insufficient balance")?;
		let new_receiver_balance =
			receiver_balance.checked_add(amount).ok_or("Overflow when adding to balance")?;

		self.set_balance(sender, new_sender_balance);
		self.set_balance(receiver, new_receiver_balance);

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn init_balances() {
		let mut balance = super::Pallet::new();

		assert_eq!(balance.get_balance("Alice"), 0);
		balance.set_balance("Alice", 100);
		assert_eq!(balance.get_balance("Alice"), 100);
		assert_eq!(balance.get_balance("Bob"), 0);
	}

	#[test]
	fn transfer_balance() {
		let mut balance = super::Pallet::new();

		balance.set_balance("alice", 100);
		let _ = balance.transfer("alice", "bob", 50);
		assert_eq!(balance.get_balance("alice"), 50);
		assert_eq!(balance.get_balance("bob"), 50);
	}

	#[test]
	fn transfer_balance_insufficient() {
		let mut balance = super::Pallet::new();

		balance.set_balance("alice", 100);
		let r = balance.transfer("alice", "bob", 150);
		assert_eq!(r, Err("Insufficient balance"));
		assert_eq!(balance.get_balance("alice"), 100);
		assert_eq!(balance.get_balance("bob"), 0);
	}

	#[test]
	fn transfer_balance_overflow() {
		let mut balance = super::Pallet::new();

		balance.set_balance("alice", 100);
		balance.set_balance("bob", u128::MAX);
		let r = balance.transfer("alice", "bob", 1);
		assert_eq!(r, Err("Overflow when adding to balance"));
		assert_eq!(balance.get_balance("alice"), 100);
		assert_eq!(balance.get_balance("bob"), u128::MAX);
	}
}
