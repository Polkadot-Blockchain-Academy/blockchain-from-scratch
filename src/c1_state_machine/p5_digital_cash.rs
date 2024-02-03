//! In this module we design another multi-user currency system. This one is not based on
//! accounts, but rather, is modelled after a paper cash system. The system tracks individual
//! cash bills. Each bill has an amount and an owner, and can be spent in its entirety.
//! When a state transition spends bills, new bills are created in lesser or equal amount.

use super::{StateMachine, User};
use std::collections::HashSet;
use std::collections::HashMap;

/// This state machine models a multi-user currency system. It tracks a set of bills in
/// circulation, and updates that set when money is transferred.
pub struct DigitalCashSystem;

/// A single bill in the digital cash system. Each bill has an owner who is allowed to spent
/// it and an amount that it is worth. It also has serial number to ensure that each bill
/// is unique.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Account {
	owner: User,
	amount: u64,
	serial: u64,
}

/// The State of a digital cash system. Primarily just the set of currently circulating bills.,
/// but also a counter for the next serial number.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Bank {
	/// The set of currently circulating bills
	bills: HashSet<Account>,
	/// The next serial number to use when a bill is created.
	next_serial: u64,
}

impl Bank {
	pub fn new() -> Self {
		Bank { bills: HashSet::<Account>::new(), next_serial: 0 }
	}

	pub fn set_serial(&mut self, serial: u64) {
		self.next_serial = serial;
	}

	pub fn next_serial(&self) -> u64 {
		self.next_serial
	}

	fn increment_serial(&mut self) {
		self.next_serial += 1
	}

	fn add_bill(&mut self, elem: Account) {
		self.bills.insert(elem);
		self.increment_serial()
	}
}

impl FromIterator<Account> for Bank {
	fn from_iter<I: IntoIterator<Item = Account>>(iter: I) -> Self {
		let mut state = Bank::new();

		for i in iter {
			state.add_bill(i)
		}
		state
	}
}

impl<const N: usize> From<[Account; N]> for Bank {
	fn from(value: [Account; N]) -> Self {
		Bank::from_iter(value)
	}
}

/// The state transitions that users can make in a digital cash system
pub enum CashTransaction {
	/// Mint a single new bill owned by the minter
	Mint { minter: User, amount: u64 },
	/// Send some money from some users to other users. The money does not all need
	/// to come from the same user, and it does not all need to go to the same user.
	/// The total amount received must be less than or equal to the amount spent.
	/// The discrepancy between the amount sent and received is destroyed. Therefore,
	/// no dedicated burn transaction is required.
	Transfer { spends: Vec<Account>, receives: Vec<Account> },
}

/// We model this system as a state machine with two possible transitions
impl StateMachine for DigitalCashSystem {
	type State = Bank;
	type Transition = CashTransaction;

	fn next_state(starting_state: &Self::State, t: &Self::Transition) -> Self::State {
		
		let mut ret_state = starting_state.clone();
		
  		match t {
		  CashTransaction::Mint{minter, amount}=>{
			  let mut creating_account = Account{
				  owner: minter.clone(),
				  amount: *amount,
				  serial: starting_state.next_serial(),
			  };
				let _ = ret_state.add_bill(creating_account);
			}
			CashTransaction::Transfer{spends, receives}=>{

				let mut spends_sum = 0;
				for i in spends{
					spends_sum += i.amount; 
				}

				let mut receives_sum = 0;
				for i in receives{
					if i.amount == u64::MAX || i.amount == 0{
						return ret_state;
					}
					receives_sum += i.amount; 
				}

				if spends_sum < receives_sum{
					return ret_state;
				}

				let mut user_spends_dict = HashMap::new();
				for i in spends{
					let user_spends = user_spends_dict.get(&i.owner).unwrap_or(&0);
						user_spends_dict.insert(i.owner, i.amount+user_spends);
				}
				let mut bank_after_spends = starting_state.clone();
				for i in user_spends_dict{
					let user = i.0;
					let user_total_spend = i.1;
					let mut enough_money_for_user = false;
					for acc in starting_state.bills.iter(){
						if user == acc.owner && user_total_spend <= acc.amount{
							let mut new_account = acc.clone();
							bank_after_spends.bills.remove(acc);
							new_account.amount -= user_total_spend; 
							if new_account.amount != 0{
								bank_after_spends.bills.insert(new_account);
							}
							
							enough_money_for_user = true;
							break;
						}
					}
					if enough_money_for_user == false{
						return ret_state;
					}
				}
				

				for i  in receives{
					let receiver = i.owner;
					let received_amount = i.amount;
					for acc in starting_state.bills.iter(){
						if receiver == acc.owner{
							let mut new_account = acc.clone();
							new_account.amount += received_amount; 
							bank_after_spends.bills.insert(new_account);
						}
					}
				}
				return bank_after_spends;
			}
			
		}
  		return ret_state;
	}
	
}

#[test]
fn sm_5_mint_new_cash() {
	let start = Bank::new();
	let end = DigitalCashSystem::next_state(
		&start,
		&CashTransaction::Mint { minter: User::Alice, amount: 20 },
	);

	let expected = Bank::from([Account { owner: User::Alice, amount: 20, serial: 0 }]);
	assert_eq!(end, expected);
}

#[test]
fn sm_5_overflow_receives_fails() {
	let start = Bank::from([Account { owner: User::Alice, amount: 42, serial: 0 }]);
	let end = DigitalCashSystem::next_state(
		&start,
		&CashTransaction::Transfer {
			spends: vec![Account { owner: User::Alice, amount: 42, serial: 0 }],
			receives: vec![
				Account { owner: User::Alice, amount: u64::MAX, serial: 1 },
				Account { owner: User::Alice, amount: 42, serial: 2 },
			],
		},
	);
	let expected = Bank::from([Account { owner: User::Alice, amount: 42, serial: 0 }]);
	assert_eq!(end, expected);
}

#[test]
fn sm_5_empty_spend_fails() {
	let start = Bank::from([Account { owner: User::Alice, amount: 20, serial: 0 }]);
	let end = DigitalCashSystem::next_state(
		&start,
		&CashTransaction::Transfer {
			spends: vec![],
			receives: vec![Account { owner: User::Alice, amount: 15, serial: 1 }],
		},
	);
	let expected = Bank::from([Account { owner: User::Alice, amount: 20, serial: 0 }]);
	assert_eq!(end, expected);
}

#[test]
fn sm_5_empty_receive_fails() {
	let start = Bank::from([Account { owner: User::Alice, amount: 20, serial: 0 }]);
	let end = DigitalCashSystem::next_state(
		&start,
		&CashTransaction::Transfer {
			spends: vec![Account { owner: User::Alice, amount: 20, serial: 0 }],
			receives: vec![],
		},
	);
	let mut expected = Bank::from([]);
	expected.set_serial(1);
	assert_eq!(end, expected);
}

#[test]
fn sm_5_output_value_0_fails() {
	let start = Bank::from([Account { owner: User::Alice, amount: 20, serial: 0 }]);
	let end = DigitalCashSystem::next_state(
		&start,
		&CashTransaction::Transfer {
			spends: vec![Account { owner: User::Alice, amount: 20, serial: 0 }],
			receives: vec![Account { owner: User::Bob, amount: 0, serial: 1 }],
		},
	);
	let expected = Bank::from([Account { owner: User::Alice, amount: 20, serial: 0 }]);
	assert_eq!(end, expected);
}

#[test]
fn sm_5_serial_number_already_seen_fails() {
	let start = Bank::from([Account { owner: User::Alice, amount: 20, serial: 0 }]);
	let end = DigitalCashSystem::next_state(
		&start,
		&CashTransaction::Transfer {
			spends: vec![Account { owner: User::Alice, amount: 20, serial: 0 }],
			receives: vec![Account { owner: User::Alice, amount: 18, serial: 0 }],
		},
	);
	let expected = Bank::from([Account { owner: User::Alice, amount: 20, serial: 0 }]);
	assert_eq!(end, expected);
}

#[test]
fn sm_5_spending_and_receiving_same_bill_fails() {
	let start = Bank::from([Account { owner: User::Alice, amount: 20, serial: 0 }]);
	let end = DigitalCashSystem::next_state(
		&start,
		&CashTransaction::Transfer {
			spends: vec![Account { owner: User::Alice, amount: 20, serial: 0 }],
			receives: vec![Account { owner: User::Alice, amount: 20, serial: 0 }],
		},
	);
	let expected = Bank::from([Account { owner: User::Alice, amount: 20, serial: 0 }]);
	assert_eq!(end, expected);
}

#[test]
fn sm_5_receiving_bill_with_incorrect_serial_fails() {
	let start = Bank::from([Account { owner: User::Alice, amount: 20, serial: 0 }]);
	let end = DigitalCashSystem::next_state(
		&start,
		&CashTransaction::Transfer {
			spends: vec![Account { owner: User::Alice, amount: 20, serial: 0 }],
			receives: vec![
				Account { owner: User::Alice, amount: 10, serial: u64::MAX },
				Account { owner: User::Bob, amount: 10, serial: 4000 },
			],
		},
	);
	let expected = Bank::from([Account { owner: User::Alice, amount: 20, serial: 0 }]);
	assert_eq!(end, expected);
}

#[test]
fn sm_5_spending_bill_with_incorrect_amount_fails() {
	let start = Bank::from([Account { owner: User::Alice, amount: 20, serial: 0 }]);
	let end = DigitalCashSystem::next_state(
		&start,
		&CashTransaction::Transfer {
			spends: vec![Account { owner: User::Alice, amount: 40, serial: 0 }],
			receives: vec![Account { owner: User::Bob, amount: 40, serial: 1 }],
		},
	);
	let expected = Bank::from([Account { owner: User::Alice, amount: 20, serial: 0 }]);
	assert_eq!(end, expected);
}

#[test]
fn sm_5_spending_same_bill_fails() {
	let start = Bank::from([Account { owner: User::Alice, amount: 40, serial: 0 }]);
	let end = DigitalCashSystem::next_state(
		&start,
		&CashTransaction::Transfer {
			spends: vec![
				Account { owner: User::Alice, amount: 40, serial: 0 },
				Account { owner: User::Alice, amount: 40, serial: 0 },
			],
			receives: vec![
				Account { owner: User::Bob, amount: 20, serial: 1 },
				Account { owner: User::Bob, amount: 20, serial: 2 },
				Account { owner: User::Alice, amount: 40, serial: 3 },
			],
		},
	);
	let expected = Bank::from([Account { owner: User::Alice, amount: 40, serial: 0 }]);
	assert_eq!(end, expected);
}

#[test]
fn sm_5_spending_more_than_bill_fails() {
	let start = Bank::from([
		Account { owner: User::Alice, amount: 40, serial: 0 },
		Account { owner: User::Charlie, amount: 42, serial: 1 },
	]);
	let end = DigitalCashSystem::next_state(
		&start,
		&CashTransaction::Transfer {
			spends: vec![
				Account { owner: User::Alice, amount: 40, serial: 0 },
				Account { owner: User::Charlie, amount: 42, serial: 1 },
			],
			receives: vec![
				Account { owner: User::Bob, amount: 20, serial: 2 },
				Account { owner: User::Bob, amount: 20, serial: 3 },
				Account { owner: User::Alice, amount: 52, serial: 4 },
			],
		},
	);
	let expected = Bank::from([
		Account { owner: User::Alice, amount: 40, serial: 0 },
		Account { owner: User::Charlie, amount: 42, serial: 1 },
	]);
	assert_eq!(end, expected);
}

#[test]
fn sm_5_spending_non_existent_bill_fails() {
	let start = Bank::from([Account { owner: User::Alice, amount: 32, serial: 0 }]);
	let end = DigitalCashSystem::next_state(
		&start,
		&CashTransaction::Transfer {
			spends: vec![Account { owner: User::Bob, amount: 1000, serial: 32 }],
			receives: vec![Account { owner: User::Bob, amount: 1000, serial: 33 }],
		},
	);
	let expected = Bank::from([Account { owner: User::Alice, amount: 32, serial: 0 }]);
	assert_eq!(end, expected);
}

#[test]
fn sm_5_spending_from_alice_to_all() {
	let start = Bank::from([Account { owner: User::Alice, amount: 42, serial: 0 }]);
	let end = DigitalCashSystem::next_state(
		&start,
		&CashTransaction::Transfer {
			spends: vec![Account { owner: User::Alice, amount: 42, serial: 0 }],
			receives: vec![
				Account { owner: User::Alice, amount: 10, serial: 1 },
				Account { owner: User::Bob, amount: 10, serial: 2 },
				Account { owner: User::Charlie, amount: 10, serial: 3 },
			],
		},
	);
	let mut expected = Bank::from([
		Account { owner: User::Alice, amount: 10, serial: 1 },
		Account { owner: User::Bob, amount: 10, serial: 2 },
		Account { owner: User::Charlie, amount: 10, serial: 3 },
	]);
	expected.set_serial(4);
	assert_eq!(end, expected);
}

#[test]
fn sm_5_spending_from_bob_to_all() {
	let start = Bank::from([Account { owner: User::Bob, amount: 42, serial: 0 }]);
	let end = DigitalCashSystem::next_state(
		&start,
		&CashTransaction::Transfer {
			spends: vec![Account { owner: User::Bob, amount: 42, serial: 0 }],
			receives: vec![
				Account { owner: User::Alice, amount: 10, serial: 1 },
				Account { owner: User::Bob, amount: 10, serial: 2 },
				Account { owner: User::Charlie, amount: 22, serial: 3 },
			],
		},
	);
	let mut expected = Bank::from([
		Account { owner: User::Alice, amount: 10, serial: 1 },
		Account { owner: User::Bob, amount: 10, serial: 2 },
		Account { owner: User::Charlie, amount: 22, serial: 3 },
	]);
	expected.set_serial(4);
	assert_eq!(end, expected);
}

#[test]
fn sm_5_spending_from_charlie_to_all() {
	let mut start = Bank::from([
		Account { owner: User::Charlie, amount: 68, serial: 54 },
		Account { owner: User::Alice, amount: 4000, serial: 58 },
	]);
	start.set_serial(59);
	let end = DigitalCashSystem::next_state(
		&start,
		&CashTransaction::Transfer {
			spends: vec![Account { owner: User::Charlie, amount: 68, serial: 54 }],
			receives: vec![
				Account { owner: User::Alice, amount: 42, serial: 59 },
				Account { owner: User::Bob, amount: 5, serial: 60 },
				Account { owner: User::Charlie, amount: 5, serial: 61 },
			],
		},
	);
	let mut expected = Bank::from([
		Account { owner: User::Alice, amount: 4000, serial: 58 },
		Account { owner: User::Alice, amount: 42, serial: 59 },
		Account { owner: User::Bob, amount: 5, serial: 60 },
		Account { owner: User::Charlie, amount: 5, serial: 61 },
	]);
	expected.set_serial(62);
	assert_eq!(end, expected);
}
