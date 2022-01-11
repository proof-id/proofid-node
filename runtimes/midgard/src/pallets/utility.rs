// --- paritytech ---
use pallet_utility::Config;

// --- proofid-network ---
use crate::*;

impl Config for Runtime {
    type Event = Event;
    type Call = Call;
    type WeightInfo = weights::pallet_utility::WeightInfo<Runtime>;
}

