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

use crate as pallet_inflation;
use crate::NegativeImbalanceOf;
use frame_support::{
	parameter_types,
	traits::{Currency, OnFinalize, OnInitialize, OnUnbalanced},
};
use pid_primitives::{
	constants::treasury::{INITIAL_PERIOD_LENGTH, INITIAL_PERIOD_REWARD_PER_BLOCK},
	AccountId, Balance, BlockHashCount, BlockNumber, Hash, Index,
};
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

pub(crate) const TREASURY_ACC: AccountId = pid_primitives::AccountId::new([0u8; 32]);

frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Event<T>},
		Inflation: pallet_inflation::{Pallet, Storage},
	}
);

parameter_types! {
	pub const SS58Prefix: u8 = 38;
}

impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = Index;
	type BlockNumber = BlockNumber;
	type Hash = Hash;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
}

parameter_types! {
	pub const ExistentialDeposit: Balance = 500;
	pub const MaxLocks: u32 = 50;
	pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for Test {
	type MaxLocks = MaxLocks;
	type MaxReserves = MaxReserves;
	type ReserveIdentifier = [u8; 8];
	type Balance = Balance;
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
}

pub struct ToBeneficiary();
impl OnUnbalanced<NegativeImbalanceOf<Test>> for ToBeneficiary {
	fn on_nonzero_unbalanced(amount: NegativeImbalanceOf<Test>) {
		// Must resolve into existing but better to be safe.
		<Test as pallet_inflation::Config>::Currency::resolve_creating(&TREASURY_ACC, amount);
	}
}

parameter_types! {
	pub const InitialPeriodLength: BlockNumber = INITIAL_PERIOD_LENGTH;
	pub const InitialPeriodReward: Balance = INITIAL_PERIOD_REWARD_PER_BLOCK;
}

impl pallet_inflation::Config for Test {
	type Currency = Balances;
	type InitialPeriodLength = InitialPeriodLength;
	type InitialPeriodReward = InitialPeriodReward;
	type Beneficiary = ToBeneficiary;
	type WeightInfo = ();
}

pub(crate) fn roll_to(n: BlockNumber) {
	while System::block_number() < n {
		<AllPallets as OnFinalize<u64>>::on_finalize(System::block_number());
		System::set_block_number(System::block_number() + 1);
		<AllPallets as OnInitialize<u64>>::on_initialize(System::block_number());
	}
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::default()
		.build_storage::<Test>()
		.unwrap()
		.into()
}
