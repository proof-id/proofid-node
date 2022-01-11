// --- paritytech ---
use pallet_did_lookup::Config;

// --- proofid-network ---
use crate::*;

parameter_types! {
	pub const DidLookupDeposit: Balance = PID;
}

impl Config for Runtime {
    type Event = Event;
    type Signature = Signature;
    type Signer = <Signature as Verify>::Signer;
    type DidIdentifier = DidIdentifier;

    type Currency = Balances;
    type Deposit = DidLookupDeposit;

    type EnsureOrigin = pallet_did::EnsureDidOrigin<DidIdentifier, AccountId>;
    type OriginSuccess = pallet_did::DidRawOrigin<AccountId, DidIdentifier>;

    type WeightInfo = weights::pallet_did_lookup::WeightInfo<Runtime>;
}