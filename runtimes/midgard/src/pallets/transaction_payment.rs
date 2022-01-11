// --- paritytech ---
use pallet_transaction_payment::Config;

use pid_primitives::fees::WeightToFee;

// --- proofid-network ---
use crate::*;

parameter_types! {
	/// This value increases the priority of `Operational` transactions by adding
    /// a "virtual tip" that's equal to the `OperationalFeeMultiplier * final_fee`.
	pub const OperationalFeeMultiplier: u8 = 5;
}

impl Config for Runtime {
    type OnChargeTransaction =
    pallet_transaction_payment::CurrencyAdapter<Balances, FeeSplit<Runtime, Treasury, ToAuthor<Runtime>>>;
    type TransactionByteFee = super::TransactionByteFee;
    type OperationalFeeMultiplier = OperationalFeeMultiplier;
    type WeightToFee = WeightToFee<Runtime>;
    type FeeMultiplierUpdate = SlowAdjustingFeeUpdate<Self>;
}
