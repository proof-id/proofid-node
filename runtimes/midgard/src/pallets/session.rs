// --- paritytech ---
use pallet_session::Config;

// --- proofid-network ---
use crate::*;

impl Config for Runtime {
    type Event = Event;
    type ValidatorId = AccountId;
    type ValidatorIdOf = ConvertInto;
    type ShouldEndSession = ParachainStaking;
    type NextSessionRotation = ParachainStaking;
    type SessionManager = ParachainStaking;
    type SessionHandler = <SessionKeys as OpaqueKeys>::KeyTypeIdProviders;
    type Keys = SessionKeys;
    type WeightInfo = weights::pallet_session::WeightInfo<Runtime>;
}