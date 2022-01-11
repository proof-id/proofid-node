// --- paritytech ---
use pallet_membership::Config;

use crate::*;

// --- proofid-network ---
use super::{MoreThanHalfCouncil, TechnicalMaxMembers};

impl Config for Runtime {
    type Event = Event;
    type AddOrigin = MoreThanHalfCouncil;
    type RemoveOrigin = MoreThanHalfCouncil;
    type SwapOrigin = MoreThanHalfCouncil;
    type ResetOrigin = MoreThanHalfCouncil;
    type PrimeOrigin = MoreThanHalfCouncil;
    type MembershipInitialized = TechnicalCommittee;
    type MembershipChanged = TechnicalCommittee;
    type MaxMembers = TechnicalMaxMembers;
    type WeightInfo = weights::pallet_membership::WeightInfo<Runtime>;
}
