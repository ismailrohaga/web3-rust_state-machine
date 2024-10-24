use std::collections::BTreeMap;

mod balances;

fn main() {
	println!("Hello, world!");

	let mut map = BTreeMap::new();
	map.insert("some_key", 1000);

	let x = map.get("some_key");
	let y = map.get("other_key");
	println!("x: {:?}", x);
	println!("x (unwrapped): {:?}", x.unwrap());
	println!("x (unwrapped_or): {:?}", x.unwrap_or(&0));
	println!("y: {:?}", y);
	// println!("y (unwrapped): {:?}", y.unwrap()); will cause a panic because of a `None` value
	println!("y (unwrapped_or): {:?}", y.unwrap_or(&404));

	let mut balance = balances::Pallet::new();
	balance.set_balance("Alice", 100);
	balance.set_balance("Bob", 200);
	balance.set_balance("Charlie", 300);
	println!("Alice balance is {:?}", balance.get_balance("Alice"));
	println!("Bob balance is {:?}", balance.get_balance("Bob"));
	println!("Charlie balance is {:?}", balance.get_balance("Charlie"));
}
