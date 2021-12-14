// --- paritytech ---
use pallet_aura::Config;

// --- proofid-network ---
use crate::{*};

parameter_types! {
    pub const MaxAuthorities: u32 = 32;
}

impl Config for Runtime {
    type AuthorityId = AuraId;
    type DisabledValidators = ();
    type MaxAuthorities = MaxAuthorities;
}