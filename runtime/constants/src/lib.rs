// Copyright 2019-2020 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

#![cfg_attr(not(feature = "std"), no_std)]

pub use self::currency::{DECIMALS, DOLLARS};

/// Balance of an account.
pub type Balance = u128;

/// Money matters.
pub mod currency {
    use super::Balance;

    // use primitives::v0::Balance;
    pub const DECIMALS: u8 = 6;
    pub const UNITS: Balance = 1_000_000;
    pub const DOLLARS: Balance = UNITS;
    // 10_000_000_000
    pub const CENTS: Balance = DOLLARS / 100;
    // 100_000_000
    pub const MILLICENTS: Balance = CENTS / 1_000; // 100_000
    //pub const SUPPLY: Balance = 1500000000_000000; // 300,000,000.000000 * 5 accounts.
}
