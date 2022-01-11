// --- paritytech ---
use pallet_ctype::Config;

// --- proofid-network ---
use crate::*;

parameter_types! {
	pub const Fee: Balance = MILLI_PID;
}

impl Config for Runtime {
    type CtypeCreatorId = AccountId;
    type Currency = Balances;
    type Fee = Fee;
    type FeeCollector = Treasury;

    #[cfg(not(feature = "runtime-benchmarks"))]
    type EnsureOrigin = pallet_did::EnsureDidOrigin<DidIdentifier, AccountId>;
    #[cfg(not(feature = "runtime-benchmarks"))]
    type OriginSuccess = pallet_did::DidRawOrigin<AccountId, DidIdentifier>;

    #[cfg(feature = "runtime-benchmarks")]
    type EnsureOrigin = EnsureSigned<DidIdentifier>;
    #[cfg(feature = "runtime-benchmarks")]
    type OriginSuccess = DidIdentifier;

    type Event = Event;
    type WeightInfo = weights::ctype::WeightInfo<Runtime>;
}
