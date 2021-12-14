// --- paritytech ---
use pallet_timestamp::Config;

// --- proofid-network ---
use proofid_common_primitives::{Moment, SLOT_DURATION};

use crate::{*, weights::pallet_timestamp::WeightInfo};

frame_support::parameter_types! {
	pub const MinimumPeriod: Moment = SLOT_DURATION / 2;
}

impl Config for Runtime {
    type Moment = u64;
    type OnTimestampSet = Aura;
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = WeightInfo<Runtime>;
}
