// --- paritytech ---
use pallet_indices::Config;

// --- proofid-network ---
use crate::*;

parameter_types! {
	pub const ExistentialDeposit: u128 = 10 * MILLI_PID;
}

impl Config for Runtime {
    type AccountIndex = Index;
    type Currency = pallet_balances::Pallet<Runtime>;
    type Deposit = ExistentialDeposit;
    type Event = Event;
    type WeightInfo = weights::pallet_indices::WeightInfo<Runtime>;
}