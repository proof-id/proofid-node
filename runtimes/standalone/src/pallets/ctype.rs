// --- paritytech ---
use pallet_ctype::Config;

// --- proofid-network ---
use crate::*;

parameter_types! {
	pub const Fee: Balance = 500;
}

impl Config for Runtime {
    type Currency = Balances;
    type Fee = Fee;
    type FeeCollector = pid_primitives::fees::ToAuthor<Runtime>;

    type CtypeCreatorId = DidIdentifier;
    type EnsureOrigin = pallet_did::EnsureDidOrigin<DidIdentifier, AccountId>;
    type OriginSuccess = pallet_did::DidRawOrigin<AccountId, DidIdentifier>;
    type Event = Event;
    type WeightInfo = ();
}