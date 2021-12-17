// --- paritytech ---
use pallet_session::Config;

// --- proofid-network ---
use crate::*;

parameter_types! {
	pub const Period: u64 = 0xFFFF_FFFF_FFFF_FFFF;
	pub const Offset: u64 = 0xFFFF_FFFF_FFFF_FFFF;
}

impl Config for Runtime {
    type Event = Event;
    type ValidatorId = AccountId;
    type ValidatorIdOf = ();
    type ShouldEndSession = pallet_session::PeriodicSessions<Period, Offset>;
    type NextSessionRotation = ();
    type SessionManager = ();
    type SessionHandler = <opaque::SessionKeys as OpaqueKeys>::KeyTypeIdProviders;
    type Keys = opaque::SessionKeys;
    type WeightInfo = ();
}