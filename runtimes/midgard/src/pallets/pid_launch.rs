// --- paritytech ---
use pallet_pid_launch::Config;

// --- proofid-network ---
use crate::*;

parameter_types! {
	pub const MaxClaims: u32 = 50;
	pub const UsableBalance: Balance = PID;
	pub const AutoUnlockBound: u32 = 100;
}

impl Config for Runtime {
    type Event = Event;
    type MaxClaims = MaxClaims;
    type UsableBalance = UsableBalance;
    type AutoUnlockBound = AutoUnlockBound;
    type WeightInfo = weights::pid_launch::WeightInfo<Runtime>;
}