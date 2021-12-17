// --- paritytech ---
use pallet_transaction_payment::Config;

// --- proofid-network ---
use crate::*;

parameter_types! {
	pub const TransactionByteFee: Balance = MICRO_PID;
	/// This value increases the priority of `Operational` transactions by adding
    /// a "virtual tip" that's equal to the `OperationalFeeMultiplier * final_fee`.
	pub const OperationalFeeMultiplier: u8 = 5;
}

impl Config for Runtime {
    type OnChargeTransaction = CurrencyAdapter<Balances, pid_primitives::fees::ToAuthor<Runtime>>;
    type TransactionByteFee = TransactionByteFee;
    type OperationalFeeMultiplier = OperationalFeeMultiplier;
    type WeightToFee = IdentityFee<Balance>;
    type FeeMultiplierUpdate = SlowAdjustingFeeUpdate<Self>;
}