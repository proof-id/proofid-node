// --- paritytech ---
use pallet_timestamp::Config;

// --- proofid-network ---
use pid_primitives::constants::SLOT_DURATION;

use crate::{*, weights::pallet_timestamp::WeightInfo};

parameter_types! {
	pub const MinimumPeriod: u64 = SLOT_DURATION / 2;
}

impl Config for Runtime {
    /// A timestamp: milliseconds since the unix epoch.
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = WeightInfo<Runtime>;
}