// --- paritytech ---
use pallet_sudo::Config;

// --- proofid-network ---
use crate::*;

impl Config for Runtime {
    type Event = Event;
    type Call = Call;
}
