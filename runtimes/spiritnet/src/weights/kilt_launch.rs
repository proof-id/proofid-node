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

//! Autogenerated weights for pid_launch
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-10-29, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("spiritnet-dev"), DB CACHE: 128

// Executed Command:
// target/release/kilt-parachain
// benchmark
// --chain=spiritnet-dev
// --steps=50
// --repeat=20
// --pallet=pid-launch
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtimes/spiritnet/src/weights/pid_launch.rs
// --template=.maintain/runtime-weight-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pid_launch`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pid_launch::WeightInfo for WeightInfo<T> {
	// Storage: KiltLaunch TransferAccount (r:0 w:1)
	fn change_transfer_account() -> Weight {
		(3_081_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: KiltLaunch UnlockingAt (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: KiltLaunch BalanceLocks (r:0 w:1)
	fn force_unlock(n: u32, ) -> Weight {
		(23_367_000 as Weight)
			// Standard Error: 23_000
			.saturating_add((28_602_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(n as Weight)))
	}
	// Storage: System Account (r:2 w:2)
	// Storage: KiltLaunch BalanceLocks (r:2 w:2)
	// Storage: Balances Locks (r:2 w:2)
	fn locked_transfer() -> Weight {
		(138_045_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	// Storage: KiltLaunch TransferAccount (r:1 w:0)
	// Storage: KiltLaunch UnownedAccount (r:1 w:1)
	// Storage: Balances Locks (r:2 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Vesting Vesting (r:2 w:2)
	// Storage: KiltLaunch BalanceLocks (r:1 w:0)
	fn migrate_genesis_account_vesting() -> Weight {
		(151_792_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	// Storage: KiltLaunch TransferAccount (r:1 w:0)
	// Storage: KiltLaunch UnownedAccount (r:1 w:1)
	// Storage: Balances Locks (r:2 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Vesting Vesting (r:1 w:0)
	// Storage: KiltLaunch BalanceLocks (r:2 w:2)
	// Storage: KiltLaunch UnlockingAt (r:1 w:1)
	fn migrate_genesis_account_locking() -> Weight {
		(155_177_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(10 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	// Storage: KiltLaunch TransferAccount (r:1 w:0)
	// Storage: KiltLaunch UnownedAccount (r:1 w:1)
	// Storage: Balances Locks (r:2 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Vesting Vesting (r:2 w:2)
	// Storage: KiltLaunch BalanceLocks (r:1 w:0)
	fn migrate_multiple_genesis_accounts_vesting(n: u32, ) -> Weight {
		(46_828_000 as Weight)
			// Standard Error: 51_000
			.saturating_add((100_849_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().reads((5 as Weight).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(n as Weight)))
	}
	// Storage: KiltLaunch TransferAccount (r:1 w:0)
	// Storage: KiltLaunch UnownedAccount (r:1 w:1)
	// Storage: Balances Locks (r:2 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Vesting Vesting (r:1 w:0)
	// Storage: KiltLaunch BalanceLocks (r:2 w:2)
	// Storage: KiltLaunch UnlockingAt (r:1 w:1)
	fn migrate_multiple_genesis_accounts_locking(n: u32, ) -> Weight {
		(52_280_000 as Weight)
			// Standard Error: 47_000
			.saturating_add((98_937_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().reads((5 as Weight).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(n as Weight)))
	}
	// Storage: KiltLaunch UnlockingAt (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: KiltLaunch BalanceLocks (r:0 w:1)
	fn on_initialize_unlock(n: u32, ) -> Weight {
		(26_534_000 as Weight)
			// Standard Error: 11_000
			.saturating_add((28_512_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(n as Weight)))
	}
	// Storage: KiltLaunch UnlockingAt (r:1 w:0)
	fn on_initialize_no_action() -> Weight {
		(4_524_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
}
