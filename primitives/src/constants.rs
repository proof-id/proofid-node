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

use frame_support::weights::{constants::WEIGHT_PER_SECOND, Weight};
use sp_runtime::{Perbill, Perquintill};

use crate::*;

/// This determines the average expected block time that we are targetting.
/// Blocks will be produced at a minimum duration defined by `SLOT_DURATION`.
/// `SLOT_DURATION` is picked up by `pallet_timestamp` which is in turn picked
/// up by `pallet_aura` to implement `fn slot_duration()`.
///
/// Change this to adjust the block time.
pub const MILLISECS_PER_BLOCK: u64 = 12_000;

pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;

/// An instant or duration in time.
pub type Moment = u64;

// Time is measured by number of blocks.
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;
// Julian year as Substrate handles it
pub const BLOCKS_PER_YEAR: BlockNumber = DAYS * 36525 / 100;

pub const MIN_VESTED_TRANSFER_AMOUNT: Balance = 100 * PID;
pub const MAX_COLLATOR_STAKE: Balance = 200_000 * PID;

/// One PID
pub const PID: Balance = 10u128.pow(6);

/// 0.001 PID
pub const MILLI_PID: Balance = PID / 1000;

/// 0.000_001 PID
pub const MICRO_PID: Balance = MILLI_PID / 1000;

// 1 in 4 blocks (on average, not counting collisions) will be primary babe
// blocks.
pub const PRIMARY_PROBABILITY: (u64, u64) = (1, 4);

/// We assume that ~10% of the block weight is consumed by `on_initalize`
/// handlers. This is used to limit the maximal weight of a single extrinsic.
pub const AVERAGE_ON_INITIALIZE_RATIO: Perbill = Perbill::from_percent(10);
/// We allow `Normal` extrinsics to fill up the block up to 75%, the rest can be
/// used by  Operational  extrinsics.
pub const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);
/// We allow for 0.5 seconds of compute with a 12 second average block time.
pub const MAXIMUM_BLOCK_WEIGHT: Weight = WEIGHT_PER_SECOND / 2;

/// Inflation configuration which is used at genesis
pub const INFLATION_CONFIG: (Perquintill, Perquintill, Perquintill, Perquintill) = (
	// max collator staking rate
	Perquintill::from_percent(40),
	// collator reward rate
	Perquintill::from_percent(10),
	// max delegator staking rate
	Perquintill::from_percent(10),
	// delegator reward rate
	Perquintill::from_percent(8),
);

/// Copied from Kusama & Polkadot runtime
pub const MAX_VESTING_SCHEDULES: u32 = 28;

pub mod attestation {
	use super::*;

	pub const ATTESTATION_DEPOSIT: Balance = PID;
}

pub mod delegation {
	use super::*;

	pub const DELEGATION_DEPOSIT: Balance = PID;
	pub const MAX_SIGNATURE_BYTE_LENGTH: u16 = 64;
	pub const MAX_PARENT_CHECKS: u32 = 5;
	pub const MAX_REVOCATIONS: u32 = 5;
	pub const MAX_REMOVALS: u32 = MAX_REVOCATIONS;
	pub const MAX_CHILDREN: u32 = 1000;
}

pub mod staking {
	use sp_runtime::Perquintill;

	use super::PID;
	#[cfg(not(feature = "fast-gov"))]
	use super::{DAYS, HOURS};
	use crate::{Balance, BlockNumber};

	/// Minimum round length is 1 hour (300 * 12 second block times)
	#[cfg(feature = "fast-gov")]
	pub const MIN_BLOCKS_PER_ROUND: BlockNumber = 10;
	#[cfg(not(feature = "fast-gov"))]
	pub const MIN_BLOCKS_PER_ROUND: BlockNumber = HOURS;

	#[cfg(feature = "fast-gov")]
	pub const DEFAULT_BLOCKS_PER_ROUND: BlockNumber = 20;
	#[cfg(not(feature = "fast-gov"))]
	pub const DEFAULT_BLOCKS_PER_ROUND: BlockNumber = 2 * HOURS;

	#[cfg(feature = "fast-gov")]
	pub const STAKE_DURATION: BlockNumber = 30;
	#[cfg(not(feature = "fast-gov"))]
	pub const STAKE_DURATION: BlockNumber = 7 * DAYS;

	#[cfg(feature = "fast-gov")]
	pub const MIN_COLLATORS: u32 = 4;
	#[cfg(not(feature = "fast-gov"))]
	pub const MIN_COLLATORS: u32 = 16;

	#[cfg(feature = "fast-gov")]
	pub const MAX_CANDIDATES: u32 = 16;
	#[cfg(not(feature = "fast-gov"))]
	pub const MAX_CANDIDATES: u32 = 75;

	pub const MIN_DELEGATOR_STAKE: Balance = 20 * PID;

	pub const NETWORK_REWARD_RATE: Perquintill = Perquintill::from_percent(10);
}

pub mod governance {
	#[cfg(feature = "fast-gov")]
	use super::MINUTES;
	#[cfg(not(feature = "fast-gov"))]
	use super::{DAYS, HOURS};
	use crate::BlockNumber;

	#[cfg(feature = "fast-gov")]
	pub const LAUNCH_PERIOD: BlockNumber = 7 * MINUTES;
	#[cfg(not(feature = "fast-gov"))]
	pub const LAUNCH_PERIOD: BlockNumber = 7 * DAYS;

	#[cfg(feature = "fast-gov")]
	pub const VOTING_PERIOD: BlockNumber = 7 * MINUTES;
	#[cfg(not(feature = "fast-gov"))]
	pub const VOTING_PERIOD: BlockNumber = 7 * DAYS;

	#[cfg(feature = "fast-gov")]
	pub const FAST_TRACK_VOTING_PERIOD: BlockNumber = 3 * MINUTES;
	#[cfg(not(feature = "fast-gov"))]
	pub const FAST_TRACK_VOTING_PERIOD: BlockNumber = 3 * HOURS;

	#[cfg(feature = "fast-gov")]
	pub const ENACTMENT_PERIOD: BlockNumber = 8 * MINUTES;
	#[cfg(not(feature = "fast-gov"))]
	pub const ENACTMENT_PERIOD: BlockNumber = DAYS;

	#[cfg(feature = "fast-gov")]
	pub const COOLOFF_PERIOD: BlockNumber = 7 * MINUTES;
	#[cfg(not(feature = "fast-gov"))]
	pub const COOLOFF_PERIOD: BlockNumber = 7 * DAYS;

	#[cfg(feature = "fast-gov")]
	pub const SPEND_PERIOD: BlockNumber = 6 * MINUTES;
	#[cfg(not(feature = "fast-gov"))]
	pub const SPEND_PERIOD: BlockNumber = 6 * DAYS;

	#[cfg(feature = "fast-gov")]
	pub const ROTATION_PERIOD: BlockNumber = 80 * MINUTES;
	#[cfg(not(feature = "fast-gov"))]
	pub const ROTATION_PERIOD: BlockNumber = 80 * HOURS;

	#[cfg(feature = "fast-gov")]
	pub const TERM_DURATION: BlockNumber = 15 * MINUTES;
	#[cfg(not(feature = "fast-gov"))]
	pub const TERM_DURATION: BlockNumber = DAYS;

	#[cfg(feature = "fast-gov")]
	pub const COUNCIL_MOTION_DURATION: BlockNumber = 4 * MINUTES;
	#[cfg(not(feature = "fast-gov"))]
	pub const COUNCIL_MOTION_DURATION: BlockNumber = 3 * DAYS;

	#[cfg(feature = "fast-gov")]
	pub const TECHNICAL_MOTION_DURATION: BlockNumber = 4 * MINUTES;
	#[cfg(not(feature = "fast-gov"))]
	pub const TECHNICAL_MOTION_DURATION: BlockNumber = 3 * DAYS;
}

pub mod did {
	use crate::BlockNumber;

	use super::{Balance, HOURS, MILLI_PID, PID};

	pub const DID_DEPOSIT: Balance = 2 * PID;
	pub const DID_FEE: Balance = 50 * MILLI_PID;
	pub const MAX_KEY_AGREEMENT_KEYS: u32 = 10;
	pub const MAX_URL_LENGTH: u32 = 200;
	// This has been reduced from the previous 100, but it might still need
	// fine-tuning depending on our needs.
	pub const MAX_PUBLIC_KEYS_PER_DID: u32 = 20;
	// At most the max number of keys - 1 for authentication
	pub const MAX_TOTAL_KEY_AGREEMENT_KEYS: u32 = MAX_PUBLIC_KEYS_PER_DID - 1;
	pub const MAX_ENDPOINT_URLS_COUNT: u32 = 3;
	pub const MAX_BLOCKS_TX_VALIDITY: BlockNumber = HOURS;

	pub const MAX_NUMBER_OF_SERVICES_PER_DID: u32 = 25;
	pub const MAX_SERVICE_ID_LENGTH: u32 = 50;
	pub const MAX_SERVICE_TYPE_LENGTH: u32 = 50;
	pub const MAX_NUMBER_OF_TYPES_PER_SERVICE: u32 = 1;
	pub const MAX_SERVICE_URL_LENGTH: u32 = 100;
	pub const MAX_NUMBER_OF_URLS_PER_SERVICE: u32 = 1;
}

pub mod treasury {
	use crate::{Balance, BlockNumber};
	use frame_support::PalletId;

	use super::{BLOCKS_PER_YEAR, PID};

	pub const INITIAL_PERIOD_LENGTH: BlockNumber = BLOCKS_PER_YEAR.saturating_mul(5);
	const YEARLY_REWARD: Balance = 2_000_000u128 * PID;
	pub const INITIAL_PERIOD_REWARD_PER_BLOCK: Balance = YEARLY_REWARD / (BLOCKS_PER_YEAR as Balance);
	pub const TREASURY_PALLET_ID: PalletId = PalletId(*b"kilt/tsy");
}
