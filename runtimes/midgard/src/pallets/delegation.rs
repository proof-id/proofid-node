// --- paritytech ---
use pallet_delegation::Config;

// --- proofid-network ---
use crate::*;

parameter_types! {
	pub const MaxSignatureByteLength: u16 = MAX_SIGNATURE_BYTE_LENGTH;
	pub const MaxParentChecks: u32 = MAX_PARENT_CHECKS;
	pub const MaxRevocations: u32 = MAX_REVOCATIONS;
	pub const MaxRemovals: u32 = MAX_REMOVALS;
	#[derive(Clone)]
	pub const MaxChildren: u32 = MAX_CHILDREN;
	pub const DelegationDeposit: Balance = DELEGATION_DEPOSIT;
}

impl Config for Runtime {
    type DelegationEntityId = DidIdentifier;
    type DelegationNodeId = Hash;

    #[cfg(not(feature = "runtime-benchmarks"))]
    type EnsureOrigin = pallet_did::EnsureDidOrigin<DidIdentifier, AccountId>;
    #[cfg(not(feature = "runtime-benchmarks"))]
    type OriginSuccess = pallet_did::DidRawOrigin<AccountId, DidIdentifier>;
    #[cfg(not(feature = "runtime-benchmarks"))]
    type DelegationSignatureVerification = pallet_did::DidSignatureVerify<Runtime>;
    #[cfg(not(feature = "runtime-benchmarks"))]
    type Signature = pallet_did::DidSignature;

    #[cfg(feature = "runtime-benchmarks")]
    type EnsureOrigin = EnsureSigned<DidIdentifier>;
    #[cfg(feature = "runtime-benchmarks")]
    type OriginSuccess = DidIdentifier;
    #[cfg(feature = "runtime-benchmarks")]
    type Signature = DummySignature;
    #[cfg(feature = "runtime-benchmarks")]
    type DelegationSignatureVerification = AlwaysVerify<AccountId, Vec<u8>, Self::Signature>;

    type Event = Event;
    type MaxSignatureByteLength = MaxSignatureByteLength;
    type MaxParentChecks = MaxParentChecks;
    type MaxRevocations = MaxRevocations;
    type MaxRemovals = MaxRemovals;
    type MaxChildren = MaxChildren;
    type WeightInfo = weights::delegation::WeightInfo<Runtime>;
    type Currency = Balances;
    type Deposit = DelegationDeposit;
}
