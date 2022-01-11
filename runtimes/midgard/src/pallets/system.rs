// --- paritytech ---
use frame_support::{traits::Contains, weights::constants::RocksDbWeight};
use frame_system::Config;

// // --- proofid-network ---
// TODO: optimize weights
// use crate::{weights::frame_system::WeightInfo, *};
use crate::*;

pub struct BaseFilter;

impl Contains<Call> for BaseFilter {
    fn contains(c: &Call) -> bool {
        !matches!(
			c,
			Call::PidLaunch(pallet_pid_launch::Call::locked_transfer { .. }) | Call::Delegation { .. }
		)
    }
}

impl Config for Runtime {
    /// The identifier used to distinguish between accounts.
    type AccountId = AccountId;
    /// The aggregated dispatch type that is available for extrinsics.
    type Call = Call;
    /// The lookup mechanism to get account ID from whatever is passed in
    /// dispatchers.
    type Lookup = AccountIdLookup<AccountId, ()>;
    /// The index type for storing how many extrinsics an account has signed.
    type Index = Index;
    /// The index type for blocks.
    type BlockNumber = BlockNumber;
    /// The type for hashing blocks and tries.
    type Hash = Hash;
    /// The hashing algorithm used.
    type Hashing = BlakeTwo256;
    /// The header type.
    type Header = pid_primitives::Header;
    /// The ubiquitous event type.
    type Event = Event;
    /// The ubiquitous origin type.
    type Origin = Origin;
    /// Maximum number of block number to block hash mappings to keep (oldest
    /// pruned first).
    type BlockHashCount = BlockHashCount;
    /// Runtime version.
    type Version = Version;
    /// Converts a module to an index of this module in the runtime.
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type DbWeight = RocksDbWeight;
    type BaseCallFilter = BaseFilter;
    type SystemWeightInfo = weights::frame_system::WeightInfo<Runtime>;
    type BlockWeights = BlockWeights;
    type BlockLength = BlockLength;
    type SS58Prefix = SS58Prefix;
    /// The set code logic
    type OnSetCode = cumulus_pallet_parachain_system::ParachainSetCode<Runtime>;
}