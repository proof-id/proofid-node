// --- paritytech ---
use pallet_pid_launch::Config;

// --- proofid-network ---
use crate::*;

parameter_types! {
	pub const MaxClaims: u32 = 50;
	pub const AutoUnlockBound: u32 = 100;
	pub const UsableBalance: Balance = PID;
}

impl Config for Runtime {
    type Event = Event;
    type MaxClaims = MaxClaims;
    type UsableBalance = UsableBalance;
    type WeightInfo = ();
    type AutoUnlockBound = AutoUnlockBound;
}