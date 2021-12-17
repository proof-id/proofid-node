// --- paritytech ---
use pallet_balances::Config;

// --- proofid-network ---
use crate::*;

parameter_types! {
	pub const ExistentialDeposit: Balance = 10 * MILLI_PID;
	pub const MaxLocks: u32 = 50;
	pub const MaxReserves: u32 = 50;
}

impl Config for Runtime {
    type MaxLocks = MaxLocks;
    type MaxReserves = MaxReserves;
    type ReserveIdentifier = [u8; 8];
    /// The type for recording an account's balance.
    type Balance = Balance;
    /// The ubiquitous event type.
    type Event = Event;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
}
