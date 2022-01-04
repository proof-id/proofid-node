// --- paritytech ---
use pallet_assets::Config;

// --- proofid-network ---
use crate::*;

parameter_types! {
    pub const AssetDeposit: Balance = 100 * 5;
    pub const ApprovalDeposit: Balance = 1 * 5;
    pub const StringLimit: u32 = 50;
    pub const MetadataDepositBase: Balance = 10 * 5;
    pub const MetadataDepositPerByte: Balance = 1 * 5;
}

impl Config for Runtime {
    type Event = Event;
    type Balance = u128;
    type AssetId = u32;
    type Currency = Balances;
    type ForceOrigin = frame_system::EnsureRoot<AccountId>;
    type AssetDeposit = AssetDeposit;
    type MetadataDepositBase = MetadataDepositBase;
    type MetadataDepositPerByte = MetadataDepositPerByte;
    type ApprovalDeposit = ApprovalDeposit;
    type StringLimit = StringLimit;
    type Freezer = ();
    type Extra = ();
    type WeightInfo = pallet_assets::weights::SubstrateWeight<Runtime>;
}