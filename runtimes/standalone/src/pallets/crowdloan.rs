// --- paritytech ---
use pallet_crowdloan::Config;

// --- proofid-network ---
use crate::*;

impl Config for Runtime {
    type Currency = Balances;
    type Vesting = Vesting;
    type Balance = Balance;
    type EnsureRegistrarOrigin = EnsureRoot<AccountId>;
    type Event = Event;
    type WeightInfo = ();
}
