// --- paritytech ---
use pallet_timestamp::Config;

// --- proofid-network ---
use pid_primitives::constants::{Moment, SLOT_DURATION};

use crate::{*, weights::pallet_timestamp::WeightInfo};

parameter_types! {
	pub const MinimumPeriod: Moment = SLOT_DURATION / 2;
}

impl Config for Runtime {
    type Moment = Moment;
    type OnTimestampSet = Aura;
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = WeightInfo<Runtime>;
}