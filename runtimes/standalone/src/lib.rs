// KILT Blockchain – https://botlabs.org
// Copyright (C) 2019-2021 BOTLabs GmbH

// The KILT Blockchain is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The KILT Blockchain is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// If you feel like getting in touch with us, you can do so at info@botlabs.org

//! The KILT runtime. This can be compiled with `#[no_std]`, ready for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]
// The `from_over_into` warning originates from `construct_runtime` macro.
#![allow(clippy::from_over_into)]

pub use frame_support::{
	ConsensusEngineId, construct_runtime,
	parameter_types,
	StorageValue,
	traits::{Currency, FindAuthor, Imbalance, KeyOwnerProofSystem, OnUnbalanced, Randomness}, weights::{
		constants::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_PER_SECOND},
		IdentityFee, Weight,
    },
};
use frame_system::EnsureRoot;
#[cfg(feature = "runtime-benchmarks")]
use frame_system::EnsureSigned;
// pub use consensus::Call as ConsensusCall;
pub use pallet_balances::Call as BalancesCall;
use pallet_grandpa::{AuthorityId as GrandpaId, AuthorityList as GrandpaAuthorityList, fg_primitives};
pub use pallet_timestamp::Call as TimestampCall;
use pallet_transaction_payment::{CurrencyAdapter, FeeDetails};
use sp_api::impl_runtime_apis;
use sp_consensus_aura::{ed25519::AuthorityId as AuraId, SlotDuration};
use sp_core::OpaqueMetadata;
use sp_runtime::{
	ApplyExtrinsicResult, create_runtime_str, generic,
	impl_opaque_keys,
	traits::{AccountIdLookup, BlakeTwo256, Block as BlockT, ConvertInto, NumberFor, OpaqueKeys, Verify},
	transaction_validity::{TransactionSource, TransactionValidity},
};
pub use sp_runtime::{Perbill, Permill};
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
use sp_std::prelude::*;
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

use pid_primitives::{
	AccountId,
	Balance,
	BlockNumber, constants::{
		attestation::ATTESTATION_DEPOSIT,
		delegation::{
			DELEGATION_DEPOSIT, MAX_CHILDREN, MAX_PARENT_CHECKS, MAX_REMOVALS, MAX_REVOCATIONS,
			MAX_SIGNATURE_BYTE_LENGTH,
        },
		did::{
			DID_DEPOSIT, DID_FEE, MAX_BLOCKS_TX_VALIDITY, MAX_ENDPOINT_URLS_COUNT, MAX_KEY_AGREEMENT_KEYS,
			MAX_NUMBER_OF_SERVICES_PER_DID, MAX_NUMBER_OF_TYPES_PER_SERVICE, MAX_NUMBER_OF_URLS_PER_SERVICE,
			MAX_PUBLIC_KEYS_PER_DID, MAX_SERVICE_ID_LENGTH, MAX_SERVICE_TYPE_LENGTH, MAX_SERVICE_URL_LENGTH,
			MAX_TOTAL_KEY_AGREEMENT_KEYS, MAX_URL_LENGTH,
        },
		MICRO_PID, MILLI_PID, MIN_VESTED_TRANSFER_AMOUNT, PID,
    }, DidIdentifier, fees::ToAuthor, Hash, Index, Signature, SlowAdjustingFeeUpdate,
};

// Make the WASM binary available
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

/// Digest item type.
pub type DigestItem = generic::DigestItem<Hash>;

pub type NegativeImbalance<T> =
<pallet_balances::Pallet<T> as Currency<<T as frame_system::Config>::AccountId>>::NegativeImbalance;

/// Opaque types. These are used by the CLI to instantiate machinery that don't
/// need to know the specifics of the runtime. They can then be made to be
/// agnostic over specific formats of data like extrinsics, allowing for them to
/// continue syncing the network through upgrades to even the core data
/// structures.
pub mod opaque {
	pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;

	use super::*;

	/// Opaque block header type.
    pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
    /// Opaque block type.
    pub type Block = generic::Block<Header, UncheckedExtrinsic>;
    /// Opaque block identifier type.
    pub type BlockId = generic::BlockId<Block>;

    impl_opaque_keys! {
		pub struct SessionKeys {
			pub aura: Aura,
			pub grandpa: Grandpa,
		}
	}
}

pub mod pallets;
pub mod weights;

/// This runtime version.
#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("proofid-node"),
    impl_name: create_runtime_str!("proofid-node"),
    authoring_version: 4,
    spec_version: 10200,
    impl_version: 0,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 2,
};

/// The version information used to identify this runtime when compiled
/// natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
    NativeVersion {
        runtime_version: VERSION,
        can_author_with: Default::default(),
    }
}

construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>} = 0,
		RandomnessCollectiveFlip: pallet_randomness_collective_flip::{Pallet, Storage} = 1,

		Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent} = 2,
		Aura: pallet_aura::{Pallet, Config<T>, Storage} = 3,
		Grandpa: pallet_grandpa::{Pallet, Call, Storage, Config, Event} = 4,
		Indices: pallet_indices::{Pallet, Call, Storage, Event<T>} = 5,
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>} = 6,
		TransactionPayment: pallet_transaction_payment::{Pallet, Storage} = 7,
		Sudo: pallet_sudo::{Pallet, Call, Config<T>, Storage, Event<T>} = 8,

		Ctype: pallet_ctype::{Pallet, Call, Storage, Event<T>} = 9,
		Attestation: pallet_attestation::{Pallet, Call, Storage, Event<T>} = 10,
		Delegation: pallet_delegation::{Pallet, Call, Storage, Event<T>} = 11,
		Did: pallet_did::{Pallet, Call, Storage, Event<T>, Origin<T>} = 12,
		DidLookup: pallet_did_lookup::{Pallet, Call, Storage, Event<T>} = 13,

		Session: pallet_session::{Pallet, Call, Storage, Event, Config<T>} = 15,
		Authorship: pallet_authorship::{Pallet, Call, Storage} = 16,

		// // Governance stuff; uncallable initially.
		// Democracy: pallet_democracy::{Module, Call, Storage, Config, Event<T>} = 25,
		// Council: pallet_collective::<Instance1>::{Module, Call, Storage, Origin<T>, Event<T>, Config<T>} = 26,
		// TechnicalCommittee: pallet_collective::<Instance2>::{Module, Call, Storage, Origin<T>, Event<T>, Config<T>} = 27,
		// ElectionsPhragmen: pallet_elections_phragmen::{Module, Call, Storage, Event<T>, Config<T>} = 28,
		// TechnicalMembership: pallet_membership::{Module, Call, Storage, Event<T>, Config<T>} = 29,
		// Treasury: pallet_treasury::{Module, Call, Storage, Config, Event<T>} = 30,

		// // System scheduler.
		// Scheduler: pallet_scheduler::{Module, Call, Storage, Event<T>} = 32,

		// Vesting. Usable initially, but removed once all vesting is finished.
		Vesting: pallet_vesting::{Pallet, Call, Storage, Event<T>, Config<T>} = 33,
		PidLaunch: pallet_pid_launch::{Pallet, Call, Storage, Event<T>, Config<T>} = 34,
		Utility: pallet_utility::{Pallet, Call, Storage, Event} = 35,
		CrowdloanContributors: pallet_crowdloan::{Pallet, Call, Storage, Event<T>, Config<T>, ValidateUnsigned} = 36,
	}
);

impl pallet_did::DeriveDidCallAuthorizationVerificationKeyRelationship for Call {
    fn derive_verification_key_relationship(&self) -> pallet_did::DeriveDidCallKeyRelationshipResult {
        fn single_key_relationship(calls: &[Call]) -> pallet_did::DeriveDidCallKeyRelationshipResult {
            let init = calls
                .get(0)
                .ok_or(pallet_did::RelationshipDeriveError::InvalidCallParameter)?
                .derive_verification_key_relationship()?;
            calls
                .iter()
                .skip(1)
                .map(Call::derive_verification_key_relationship)
                .try_fold(init, |acc, next| {
                    if Ok(acc) == next {
                        Ok(acc)
                    } else {
                        Err(pallet_did::RelationshipDeriveError::InvalidCallParameter)
                    }
                })
        }
        match self {
            Call::Attestation { .. } => Ok(pallet_did::DidVerificationKeyRelationship::AssertionMethod),
            Call::Ctype { .. } => Ok(pallet_did::DidVerificationKeyRelationship::AssertionMethod),
            Call::Delegation { .. } => Ok(pallet_did::DidVerificationKeyRelationship::CapabilityDelegation),
            // DID creation is not allowed through the DID proxy.
            Call::Did(pallet_did::Call::create { .. }) => Err(pallet_did::RelationshipDeriveError::NotCallableByDid),
            Call::Did { .. } => Ok(pallet_did::DidVerificationKeyRelationship::Authentication),
            Call::Utility(pallet_utility::Call::batch { calls }) => single_key_relationship(&calls[..]),
            Call::Utility(pallet_utility::Call::batch_all { calls }) => single_key_relationship(&calls[..]),
            #[cfg(not(feature = "runtime-benchmarks"))]
            _ => Err(pallet_did::RelationshipDeriveError::NotCallableByDid),
            // By default, returns the authentication key
            #[cfg(feature = "runtime-benchmarks")]
            _ => Ok(pallet_did::DidVerificationKeyRelationship::Authentication),
        }
    }

    // Always return a System::remark() extrinsic call
    #[cfg(feature = "runtime-benchmarks")]
    fn get_call_for_did_call_benchmark() -> Self {
        Call::System(frame_system::Call::remark { remark: vec![] })
    }
}

/// The address format for describing accounts.
pub type Address = sp_runtime::MultiAddress<AccountId, ()>;
/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// A Block signed with a Justification
pub type SignedBlock = generic::SignedBlock<Block>;
/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;
/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
    frame_system::CheckSpecVersion<Runtime>,
    frame_system::CheckTxVersion<Runtime>,
    frame_system::CheckGenesis<Runtime>,
    frame_system::CheckEra<Runtime>,
    frame_system::CheckNonce<Runtime>,
    frame_system::CheckWeight<Runtime>,
    pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);
/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<Address, Call, Signature, SignedExtra>;
/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, Call, SignedExtra>;
/// Executive: handles dispatch to the various Pallets.
pub type Executive =
frame_executive::Executive<Runtime, Block, frame_system::ChainContext<Runtime>, Runtime, AllPallets>;

impl_runtime_apis! {
	impl sp_api::Core<Block> for Runtime {
		fn version() -> RuntimeVersion {
			VERSION
		}

		fn execute_block(block: Block) {
			Executive::execute_block(block);
		}

		fn initialize_block(header: &<Block as BlockT>::Header) {
			Executive::initialize_block(header)
		}
	}

	impl sp_api::Metadata<Block> for Runtime {
		fn metadata() -> OpaqueMetadata {
			OpaqueMetadata::new(Runtime::metadata().into())
		}
	}

	impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Index> for Runtime {
		fn account_nonce(account: AccountId) -> Index {
			frame_system::Pallet::<Runtime>::account_nonce(&account)
		}
	}

	impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance> for Runtime {
		fn query_info(
			uxt: <Block as BlockT>::Extrinsic,
			len: u32,
		) -> pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<Balance> {
			TransactionPayment::query_info(uxt, len)
		}

		fn query_fee_details(uxt: <Block as BlockT>::Extrinsic, len: u32) -> FeeDetails<Balance> {
			TransactionPayment::query_fee_details(uxt, len)
		}
	}

	impl sp_block_builder::BlockBuilder<Block> for Runtime {
		fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
			Executive::apply_extrinsic(extrinsic)
		}

		fn finalize_block() -> <Block as BlockT>::Header {
			Executive::finalize_block()
		}

		fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
			data.create_extrinsics()
		}

		fn check_inherents(
			block: Block,
			data: sp_inherents::InherentData,
		) -> sp_inherents::CheckInherentsResult {
			data.check_extrinsics(&block)
		}
	}

	impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
		fn validate_transaction(
			source: TransactionSource,
			tx: <Block as BlockT>::Extrinsic,
			block_hash: <Block as BlockT>::Hash,
		) -> TransactionValidity {
			Executive::validate_transaction(source, tx, block_hash)
		}
	}
	impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
		fn offchain_worker(header: &<Block as BlockT>::Header) {
			Executive::offchain_worker(header)
		}
	}

	impl sp_session::SessionKeys<Block> for Runtime {
		fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
			opaque::SessionKeys::generate(seed)
		}

		fn decode_session_keys(
			encoded: Vec<u8>,
		) -> Option<Vec<(Vec<u8>, sp_core::crypto::KeyTypeId)>> {
			opaque::SessionKeys::decode_into_raw_public_keys(&encoded)
		}
	}

	impl sp_consensus_aura::AuraApi<Block, AuraId> for Runtime {
		fn slot_duration() -> SlotDuration {
			SlotDuration::from_millis(Aura::slot_duration())
		}

		fn authorities() -> Vec<AuraId> {
			Aura::authorities().into_inner()
		}
	}

	impl fg_primitives::GrandpaApi<Block> for Runtime {
		fn current_set_id() -> fg_primitives::SetId {
			Grandpa::current_set_id()
		}

		fn grandpa_authorities() -> GrandpaAuthorityList {
			Grandpa::grandpa_authorities()
		}

		fn submit_report_equivocation_unsigned_extrinsic(
			_equivocation_proof: fg_primitives::EquivocationProof<
				<Block as BlockT>::Hash,
				NumberFor<Block>,
			>,
			_key_owner_proof: fg_primitives::OpaqueKeyOwnershipProof,
		) -> Option<()> {
			None
		}

		fn generate_key_ownership_proof(
			_set_id: fg_primitives::SetId,
			_authority_id: GrandpaId,
		) -> Option<fg_primitives::OpaqueKeyOwnershipProof> {
			// NOTE: this is the only implementation possible since we've
			// defined our key owner proof type as a bottom type (i.e. a type
			// with no values).
			None
		}
	}

	#[cfg(feature = "runtime-benchmarks")]
	impl frame_benchmarking::Benchmark<Block> for Runtime {
		fn benchmark_metadata(extra: bool) -> (
			Vec<frame_benchmarking::BenchmarkList>,
			Vec<frame_support::traits::StorageInfo>,
		) {
			use frame_benchmarking::{list_benchmark, Benchmarking, BenchmarkList};
			use frame_support::traits::StorageInfoTrait;
			use frame_system_benchmarking::Pallet as SystemBench;

			let mut list = Vec::<BenchmarkList>::new();

			list_benchmark!(list, extra, frame_system, SystemBench::<Runtime>);
			list_benchmark!(list, extra, pallet_balances, Balances);
			list_benchmark!(list, extra, pallet_timestamp, Timestamp);

			list_benchmark!(list, extra, frame_system, SystemBench::<Runtime>);
			list_benchmark!(list, extra, pallet_balances, Balances);
			list_benchmark!(list, extra, pallet_timestamp, Timestamp);
			list_benchmark!(list, extra, pid_launch, KiltLaunch);
			list_benchmark!(list, extra, pallet_vesting, Vesting);

			list_benchmark!(list, extra, pallet_did, Did);
			list_benchmark!(list, extra, pallet_ctype, Ctype);
			list_benchmark!(list, extra, pallet_crowdloan, CrowdloanContributors);
			list_benchmark!(list, extra, pallet_delegation, Delegation);
			list_benchmark!(list, extra, pallet_attestation, Attestation);

			let storage_info = AllPalletsWithSystem::storage_info();

			(list, storage_info)
		}

		fn dispatch_benchmark(
			config: frame_benchmarking::BenchmarkConfig
		) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
			use frame_benchmarking::{Benchmarking, BenchmarkBatch, add_benchmark, TrackedStorageKey};

			use frame_system_benchmarking::Pallet as SystemBench;
			impl frame_system_benchmarking::Config for Runtime {}

			let whitelist: Vec<TrackedStorageKey> = vec![
				// Block Number
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef702a5c1b19ab7a04f536c519aca4983ac")
					.to_vec().into(),
				// Total Issuance
				hex_literal::hex!("c2261276cc9d1f8598ea4b6a74b15c2f57c875e4cff74148e4628f264b974c80")
					.to_vec().into(),
				// Execution Phase
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef7ff553b5a9862a516939d82b3d3d8661a")
					.to_vec().into(),
				// Event Count
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef70a98fdbe9ce6c55837576c60c7af3850")
					.to_vec().into(),
				// System Events
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7")
					.to_vec().into(),
				// KiltLaunch transfer account
				hex_literal::hex!("6a3c793cec9dbe330b349dc4eea6801090f5e71f53b1b41ad11afb4a313a282c").to_vec().into(),
			];

			let mut batches = Vec::<BenchmarkBatch>::new();
			let params = (&config, &whitelist);

			add_benchmark!(params, batches, frame_system, SystemBench::<Runtime>);
			add_benchmark!(params, batches, pallet_balances, Balances);
			add_benchmark!(params, batches, pallet_timestamp, Timestamp);
			add_benchmark!(params, batches, pid_launch, KiltLaunch);
			add_benchmark!(params, batches, pallet_vesting, Vesting);

			add_benchmark!(params, batches, pallet_did, Did);
			add_benchmark!(params, batches, pallet_ctype, Ctype);
			add_benchmark!(params, batches, pallet_crowdloan, CrowdloanContributors);
			add_benchmark!(params, batches, pallet_delegation, Delegation);
			add_benchmark!(params, batches, pallet_attestation, Attestation);

			if batches.is_empty() { return Err("Benchmark not found for this pallet.".into()) }
			Ok(batches)
		}
	}

	#[cfg(feature = "try-runtime")]
	impl frame_try_runtime::TryRuntime<Block> for Runtime {
		fn on_runtime_upgrade() -> (Weight, Weight) {
			log::info!("try-runtime::on_runtime_upgrade standalone runtime.");
			let weight = Executive::try_runtime_upgrade().map_err(|err|{
				log::info!("try-runtime::on_runtime_upgrade failed with: {:?}", err);
				err
			}).unwrap();
			(weight, BlockWeights::get().max_block)
		}
		fn execute_block_no_check(block: Block) -> Weight {
			Executive::execute_block_no_check(block)
		}
	}
}
