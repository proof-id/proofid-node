// --- paritytech ---
use pallet_attestation::Config;

// --- proofid-network ---
use crate::*;

parameter_types! {
	pub const MaxDelegatedAttestations: u32 = 1000;
	pub const AttestationDeposit: Balance = ATTESTATION_DEPOSIT;
}

impl Config for Runtime {
    type EnsureOrigin = pallet_did::EnsureDidOrigin<DidIdentifier, AccountId>;
    type OriginSuccess = pallet_did::DidRawOrigin<DidIdentifier, AccountId>;
    type Event = Event;
    type WeightInfo = ();
    type Currency = Balances;
    type Deposit = AttestationDeposit;
    type MaxDelegatedAttestations = MaxDelegatedAttestations;
}