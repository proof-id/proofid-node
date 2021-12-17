// --- paritytech ---
use pallet_vesting::Config;

// --- proofid-network ---
use crate::*;

parameter_types! {
	pub const MinVestedTransfer: Balance = MIN_VESTED_TRANSFER_AMOUNT;
}

impl Config for Runtime {
    type Event = Event;
    type Currency = Balances;
    type BlockNumberToBalance = ConvertInto;
    // disable vested transfers by setting min amount to max balance
    type MinVestedTransfer = MinVestedTransfer;
    const MAX_VESTING_SCHEDULES: u32 = pid_primitives::constants::MAX_VESTING_SCHEDULES;
    type WeightInfo = ();
}