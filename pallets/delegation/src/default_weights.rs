// PID Blockchain – https://botlabs.org
// Copyright (C) 2019-2021 BOTLabs GmbH

// The PID Blockchain is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The PID Blockchain is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// If you feel like getting in touch with us, you can do so at info@botlabs.org

//! Autogenerated weights for delegation
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-11-08, STEPS: {{cmd.steps}}\, REPEAT: {{cmd.repeat}}\, LOW RANGE: {{cmd.lowest_range_values}}\, HIGH RANGE: {{cmd.highest_range_values}}\
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/kilt-parachain
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=delegation
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=pallets/delegation/src/default_weights.rs
// --template=.maintain/weight-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for delegation.
pub trait WeightInfo {
	fn create_hierarchy() -> Weight;
	fn add_delegation() -> Weight;
	fn revoke_delegation_root_child(r: u32, c: u32, ) -> Weight;
	fn revoke_delegation_leaf(r: u32, c: u32, ) -> Weight;
	fn remove_delegation(r: u32, ) -> Weight;
	fn reclaim_deposit(r: u32, ) -> Weight;
}

/// Weights for delegation using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Delegation DelegationHierarchies (r:1 w:1)
	// Storage: Ctype Ctypes (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Delegation DelegationNodes (r:0 w:1)
	fn create_hierarchy() -> Weight {
		(47_416_000_u64)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: Delegation DelegationNodes (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn add_delegation() -> Weight {
		(57_220_000_u64)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: Delegation DelegationNodes (r:1 w:1)
	// Storage: Delegation DelegationHierarchies (r:1 w:0)
	fn revoke_delegation_root_child(r: u32, c: u32, ) -> Weight {
		(22_152_000_u64)
			// Standard Error: 55_000
			.saturating_add((18_681_000_u64).saturating_mul(r as Weight))
			// Standard Error: 55_000
			.saturating_add((95_000_u64).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r as Weight)))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(r as Weight)))
	}
	// Storage: Delegation DelegationNodes (r:6 w:1)
	// Storage: Delegation DelegationHierarchies (r:1 w:0)
	fn revoke_delegation_leaf(r: u32, c: u32, ) -> Weight {
		(39_250_000_u64)
			// Standard Error: 34_000
			.saturating_add((147_000_u64).saturating_mul(r as Weight))
			// Standard Error: 34_000
			.saturating_add((5_748_000_u64).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(c as Weight)))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: Delegation DelegationNodes (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	// Storage: Delegation DelegationHierarchies (r:1 w:1)
	fn remove_delegation(r: u32, ) -> Weight {
		(61_171_000_u64)
			// Standard Error: 99_000
			.saturating_add((37_949_000_u64).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(r as Weight)))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(r as Weight)))
	}
	// Storage: Delegation DelegationNodes (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	// Storage: Delegation DelegationHierarchies (r:0 w:1)
	fn reclaim_deposit(r: u32, ) -> Weight {
		(49_884_000_u64)
			// Standard Error: 64_000
			.saturating_add((38_418_000_u64).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(r as Weight)))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(r as Weight)))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn create_hierarchy() -> Weight {
		(47_416_000_u64)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	fn add_delegation() -> Weight {
		(57_220_000_u64)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	fn revoke_delegation_root_child(r: u32, c: u32, ) -> Weight {
		(22_152_000_u64)
			// Standard Error: 55_000
			.saturating_add((18_681_000_u64).saturating_mul(r as Weight))
			// Standard Error: 55_000
			.saturating_add((95_000_u64).saturating_mul(c as Weight))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(r as Weight)))
			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(r as Weight)))
	}
	fn revoke_delegation_leaf(r: u32, c: u32, ) -> Weight {
		(39_250_000_u64)
			// Standard Error: 34_000
			.saturating_add((147_000_u64).saturating_mul(r as Weight))
			// Standard Error: 34_000
			.saturating_add((5_748_000_u64).saturating_mul(c as Weight))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(c as Weight)))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn remove_delegation(r: u32, ) -> Weight {
		(61_171_000_u64)
			// Standard Error: 99_000
			.saturating_add((37_949_000_u64).saturating_mul(r as Weight))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().reads((2_u64).saturating_mul(r as Weight)))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
			.saturating_add(RocksDbWeight::get().writes((2_u64).saturating_mul(r as Weight)))
	}
	fn reclaim_deposit(r: u32, ) -> Weight {
		(49_884_000_u64)
			// Standard Error: 64_000
			.saturating_add((38_418_000_u64).saturating_mul(r as Weight))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().reads((2_u64).saturating_mul(r as Weight)))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
			.saturating_add(RocksDbWeight::get().writes((2_u64).saturating_mul(r as Weight)))
	}
}
