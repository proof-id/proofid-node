// --- paritytech ---
use pallet_identity::Config;

// --- proofid-network ---
use crate::{*, weights::pallet_identity::WeightInfo};

frame_support::parameter_types! {
	// Minimum 100 bytes/PID deposited (1 MILLI/byte)
	pub const BasicDeposit: Balance = deposit(1, 258);
	pub const FieldDeposit: Balance = deposit(0, 66);
	pub const SubAccountDeposit: Balance = deposit(1, 53);
	pub const MaxSubAccounts: u32 = 100;
	pub const MaxAdditionalFields: u32 = 100;
	pub const MaxRegistrars: u32 = 20;
}

impl Config for Runtime {
    type Event = Event;
    type Currency = Balances;
    type BasicDeposit = BasicDeposit;
    type FieldDeposit = FieldDeposit;
    type SubAccountDeposit = SubAccountDeposit;
    type MaxSubAccounts = MaxSubAccounts;
    type MaxAdditionalFields = MaxAdditionalFields;
    type MaxRegistrars = MaxRegistrars;
    type Slashed = ();
    type ForceOrigin = EnsureRoot<AccountId>;
    type RegistrarOrigin = EnsureRoot<AccountId>;
    type WeightInfo = WeightInfo<Runtime>;
}