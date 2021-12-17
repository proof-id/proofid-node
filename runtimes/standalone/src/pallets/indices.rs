// --- paritytech ---
use pallet_indices::Config;

// --- proofid-network ---
use crate::*;

parameter_types! {
	pub const Deposit: Balance = 1_000;
}

impl Config for Runtime {
    type AccountIndex = Index;
    type Currency = Balances;
    type Deposit = Deposit;
    type Event = Event;
    type WeightInfo = ();
}
