use pallet_transaction_payment::{Config, CurrencyAdapter};

// --- proofid-network ---
use crate::*;

parameter_types! {
    pub const TransactionByteFee: Balance = 1;
    pub OperationalFeeMultiplier: u8 = 5;
}

impl Config for Runtime {
    type OnChargeTransaction = CurrencyAdapter<Balances, ()>;
    type TransactionByteFee = TransactionByteFee;
    type OperationalFeeMultiplier = OperationalFeeMultiplier;
    type WeightToFee = IdentityFee<Balance>;
    type FeeMultiplierUpdate = ();
}