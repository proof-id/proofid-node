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

//! //! Autogenerated weights for pallet_utility
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-06-10, STEPS: {{cmd.steps}}\, REPEAT: {{cmd.repeat}}\, LOW RANGE:
//! {{cmd.lowest_range_values}}\, HIGH RANGE: {{cmd.highest_range_values}}\
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN:
//! Some("midgard-dev"), DB CACHE: 128

// Executed Command:
// /home/willi/proofid-node/target/release/kilt-parachain
// benchmark
// --chain=midgard-dev
// --execution=wasm
// --wasm-execution=Compiled
// --heap-pages=4096
// --extrinsic=*
// --pallet=pallet_utility
// --steps=50
// --repeat=20
// --output
// ../../runtimes/midgard/src/weights/pallet_utility.rs
// --template
// ../../.maintain/weight-template.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weights for pallet_utility using the Substrate node and recommended
/// hardware.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_utility::WeightInfo for WeightInfo<T> {
	fn batch(c: u32) -> Weight {
		(24_426_000_u64)
			// Standard Error: 1_000
			.saturating_add((477_000_u64).saturating_mul(c as Weight))
	}
	fn as_derivative() -> Weight {
		(4_419_000_u64)
	}
	fn batch_all(c: u32) -> Weight {
		(25_118_000_u64)
			// Standard Error: 2_000
			.saturating_add((1_059_000_u64).saturating_mul(c as Weight))
	}
}
