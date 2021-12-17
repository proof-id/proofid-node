// --- paritytech ---
use pallet_authorship::Config;

// --- proofid-network ---
use crate::*;

parameter_types! {
	pub const UncleGenerations: u32 = 0;
}

impl Config for Runtime {
    type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Aura>;
    type UncleGenerations = UncleGenerations;
    type FilterUncle = ();
    type EventHandler = ();
}