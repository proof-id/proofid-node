// --- paritytech ---
use pallet_aura::Config;

use pid_primitives::constants::staking::MAX_CANDIDATES;

// --- proofid-network ---
use crate::*;

parameter_types! {
	pub const MaxAuthorities: u32  = MAX_CANDIDATES;
}

impl Config for Runtime {
    type AuthorityId = AuraId;
    type DisabledValidators = ();
    type MaxAuthorities = MaxAuthorities;
}
