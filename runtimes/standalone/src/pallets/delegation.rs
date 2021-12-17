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
    #[cfg(not(feature = "runtime-benchmarks"))]
    type Signature = pallet_did::DidSignature;
    #[cfg(not(feature = "runtime-benchmarks"))]
    type DelegationSignatureVerification = pallet_did::DidSignatureVerify<Self>;

    #[cfg(feature = "runtime-benchmarks")]
    type Signature = pid_primitives::benchmarks::DummySignature;
    #[cfg(feature = "runtime-benchmarks")]
    type DelegationSignatureVerification = pid_support::signature::AlwaysVerify<AccountId, Vec<u8>, Self::Signature>;

    type DelegationEntityId = DidIdentifier;
    type DelegationNodeId = Hash;
    type EnsureOrigin = pallet_did::EnsureDidOrigin<DidIdentifier, AccountId>;
    type OriginSuccess = pallet_did::DidRawOrigin<AccountId, DidIdentifier>;
    type Event = Event;
    type MaxSignatureByteLength = MaxSignatureByteLength;
    type MaxParentChecks = MaxParentChecks;
    type MaxRevocations = MaxRevocations;
    type MaxRemovals = MaxRemovals;
    type MaxChildren = MaxChildren;
    type WeightInfo = ();
    type Currency = Balances;
    type Deposit = DelegationDeposit;
}
