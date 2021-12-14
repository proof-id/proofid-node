//! Proofid types shared between the runtime and the node-side code.

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]

// --- paritytech ---
use sp_core::H256;
use sp_runtime::{
	generic,
	traits::{BlakeTwo256, IdentifyAccount, Verify},
	MultiSignature, OpaqueExtrinsic,
};

/// Index of a transaction in the chain.
pub type Index = u32;

/// An index to a block.
/// 32-bits will allow for 136 years of blocks assuming 1 block per second.
pub type BlockNumber = u32;

/// An instant or duration in time.
pub type Moment = u64;

/// Alias to type for a signature for a transaction on the relay chain. This allows one of several
/// kinds of underlying crypto to be used, so isn't a fixed size when encoded.
pub type Signature = MultiSignature;

/// Alias to the public key used for this chain, actually a `MultiSigner`. Like the signature, this
/// also isn't a fixed size when encoded, as different cryptos have different size public keys.
pub type AccountPublic = <Signature as Verify>::Signer;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// The type for looking up accounts. We don't expect more than 4 billion of them.
pub type AccountIndex = u32;

/// A hash of some data used by the relay chain.
pub type Hash = H256;

/// Hashing algorithm used by the chain.
pub type Hashing = BlakeTwo256;

/// Index of a transaction in the relay chain. 32-bit should be plenty.
pub type Nonce = u32;

/// The balance of an account.
/// 128-bits (or 38 significant decimal figures) will allow for 10m currency (10^7) at a resolution
/// to all for one second's worth of an annualised 50% reward be paid to a unit holder (10^11 unit
/// denomination), or 10^18 total atomic units, to grow at 50%/year for 51 years (10^9 multiplier)
/// for an eventual total of 10^27 units (27 significant decimal figures).
/// We round denomination to 10^12 (12 sdf), and leave the other redundancy at the upper end so
/// that 32 bits may be multiplied with a balance in 128 bits without worrying about overflow.
pub type Balance = u128;

/// The power of an account.
pub type Power = u32;

/// Header type.
pub type Header = generic::Header<BlockNumber, Hashing>;

/// Block type.
pub type OpaqueBlock = generic::Block<Header, OpaqueExtrinsic>;

/// 1 in u128.
pub const NANO: Balance = 1;
/// 1_000 in u128.
pub const MICRO: Balance = 1_000 * NANO;
/// 1_000_000 in u128.
pub const MILLI: Balance = 1_000 * MICRO;
/// 1_000_000_000 in u128.
pub const COIN: Balance = 1_000 * MILLI;

/// The hard cap of RING.
pub const RING_HARD_CAP: Balance = 10_000_000_000 * COIN;
/// The amount of total power.
pub const TOTAL_POWER: Power = 1_000_000_000;

/// Deposit calculator for Proofid.
pub const fn deposit(items: u32, bytes: u32) -> Balance {
	items as Balance * 20 * MILLI + (bytes as Balance) * 100 * NANO
}

/// This determines the average expected block time that we are targeting.
/// Blocks will be produced at a minimum duration defined by `SLOT_DURATION`.
/// `SLOT_DURATION` is picked up by `pallet_timestamp` which is in turn picked
/// up by `pallet_aura` to implement `fn slot_duration()`.
///
/// Change this to adjust the block time.
pub const MILLISECS_PER_BLOCK: Moment = 6000;

/// Time is measured by number of blocks.
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
/// Hour in Proofid.
pub const HOURS: BlockNumber = 60 * MINUTES;
/// Day in Proofid.
pub const DAYS: BlockNumber = 24 * HOURS;

/// Slot duration in Proofid.
// NOTE: Currently it is not possible to change the slot duration after the chain has started.
//       Attempting to do so will brick block production.
pub const SLOT_DURATION: Moment = MILLISECS_PER_BLOCK;

/// Session length of Proofid.
pub const SESSION_BLOCKS_PER_SESSION: BlockNumber = 4 * HOURS;
/// Era length of Proofid.
pub const SESSION_SESSIONS_PER_ERA: BlockNumber = 6;
