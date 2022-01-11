// --- paritytech ---
use pallet_aura::Config;

// --- proofid-network ---
use crate::*;

parameter_types! {
	pub const MaxAuthorities: u32 = constants::staking::MAX_CANDIDATES;
}

impl Config for Runtime {
    type AuthorityId = AuthorityId;
    //TODO: handle disabled validators
    type DisabledValidators = ();
    type MaxAuthorities = MaxAuthorities;
}
