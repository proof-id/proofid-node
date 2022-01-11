// --- paritytech ---
use pallet_attestation::Config;

// --- proofid-network ---
use crate::*;

parameter_types! {
	pub const MaxDelegatedAttestations: u32 = 1000;
	pub const AttestationDeposit: Balance = ATTESTATION_DEPOSIT;
}

impl Config for Runtime {
    #[cfg(not(feature = "runtime-benchmarks"))]
    type EnsureOrigin = pallet_did::EnsureDidOrigin<DidIdentifier, AccountId>;
    #[cfg(not(feature = "runtime-benchmarks"))]
    type OriginSuccess = pallet_did::DidRawOrigin<AccountId, DidIdentifier>;

    #[cfg(feature = "runtime-benchmarks")]
    type EnsureOrigin = EnsureSigned<DidIdentifier>;
    #[cfg(feature = "runtime-benchmarks")]
    type OriginSuccess = DidIdentifier;

    type Event = Event;
    type WeightInfo = weights::attestation::WeightInfo<Runtime>;

    type Currency = Balances;
    type Deposit = AttestationDeposit;
    type MaxDelegatedAttestations = MaxDelegatedAttestations;
}