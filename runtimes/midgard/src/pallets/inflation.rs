// --- paritytech ---
use pallet_inflation::Config;

// --- proofid-network ---
use crate::*;

parameter_types! {
	pub const InitialPeriodLength: BlockNumber = INITIAL_PERIOD_LENGTH;
	pub const InitialPeriodReward: Balance = INITIAL_PERIOD_REWARD_PER_BLOCK;
}

impl Config for Runtime {
    type Currency = Balances;
    type InitialPeriodLength = InitialPeriodLength;
    type InitialPeriodReward = InitialPeriodReward;
    type Beneficiary = Treasury;
    type WeightInfo = weights::pallet_inflation::WeightInfo<Runtime>;
}