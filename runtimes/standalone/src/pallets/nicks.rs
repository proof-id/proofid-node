// --- paritytech ---
use pallet_nicks::Config;

// --- proofid-network ---
use crate::*;

// Add this code block to your template for Nicks:
parameter_types! {
    // Choose a fee that incentivizes desireable behavior.
    pub const NickReservationFee: u128 = 100;
    pub const MinNickLength: u32 = 8;
    // Maximum bounds on storage are important to secure your chain.
    pub const MaxNickLength: u32 = 32;
}

impl Config for Runtime {
    // The Balances pallet implements the ReservableCurrency trait.
    // `Balances` is defined in `construct_runtime!` macro. See below.
    // https://docs.substrate.io/rustdocs/latest/pallet_balances/index.html#implementations-2
    type Currency = Balances;

    // Use the NickReservationFee from the parameter_types block.
    type ReservationFee = NickReservationFee;

    // No action is taken when deposits are forfeited.
    type Slashed = ();

    // Configure the FRAME System Root origin as the Nick pallet admin.
    // https://docs.substrate.io/rustdocs/latest/frame_system/enum.RawOrigin.html#variant.Root
    type ForceOrigin = frame_system::EnsureRoot<AccountId>;

    // Use the MinNickLength from the parameter_types block.
    type MinLength = MinNickLength;

    // Use the MaxNickLength from the parameter_types block.
    type MaxLength = MaxNickLength;

    // The ubiquitous event type.
    type Event = Event;
}