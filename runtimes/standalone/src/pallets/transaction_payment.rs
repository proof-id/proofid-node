// --- paritytech ---
use pallet_transaction_payment::Config;
use pid_primitives::fees::WeightToFee;

// --- proofid-network ---
use crate::*;

parameter_types! {
	pub const TransactionByteFee: Balance = 0;
	/// This value increases the priority of `Operational` transactions by adding
    /// a "virtual tip" that's equal to the `OperationalFeeMultiplier * final_fee`.
	pub const OperationalFeeMultiplier: u8 = 0;
}

impl Config for Runtime {
    type OnChargeTransaction = CurrencyAdapter<Balances, ()>;// CurrencyAdapter<Balances, pid_primitives::fees::ToAuthor<Runtime>>;
    type TransactionByteFee = TransactionByteFee;
    type OperationalFeeMultiplier = OperationalFeeMultiplier;
    type WeightToFee = WeightToFee<Runtime>;// IdentityFee<Balance>;
    type FeeMultiplierUpdate = (); // SlowAdjustingFeeUpdate<Self>;
}