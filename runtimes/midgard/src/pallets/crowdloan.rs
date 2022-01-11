// --- paritytech ---
use pallet_crowdloan::Config;

use crate::*;

// --- proofid-network ---
use super::MoreThanHalfCouncil;

impl Config for Runtime {
    type Currency = Balances;
    type Vesting = Vesting;
    type Balance = Balance;
    type EnsureRegistrarOrigin = MoreThanHalfCouncil;
    type Event = Event;
    type WeightInfo = weights::crowdloan::WeightInfo<Runtime>;
}
