// PID Blockchain – https://botlabs.org
// Copyright (C) 2019-2021 BOTLabs GmbH

// The PID Blockchain is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The PID Blockchain is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// If you feel like getting in touch with us, you can do so at info@botlabs.org

use sp_runtime::traits::Zero;

use crate::{mock::*, pallet::Config};

#[test]
fn during_initial_period() {
	new_test_ext().execute_with(|| {
		assert!(Balances::free_balance(&TREASURY_ACC).is_zero());
		assert!(<Test as Config>::Currency::total_issuance().is_zero());

		roll_to(1);
		assert_eq!(
			Balances::free_balance(&TREASURY_ACC),
			<Test as Config>::InitialPeriodReward::get()
		);
		assert_eq!(
			<Test as Config>::Currency::total_issuance(),
			<Test as Config>::InitialPeriodReward::get()
		);

		roll_to(2);
		assert_eq!(
			Balances::free_balance(&TREASURY_ACC),
			2 * <Test as Config>::InitialPeriodReward::get()
		);
		assert_eq!(
			<Test as Config>::Currency::total_issuance(),
			2 * <Test as Config>::InitialPeriodReward::get()
		);

		roll_to(100);
		assert_eq!(
			Balances::free_balance(&TREASURY_ACC),
			100 * <Test as Config>::InitialPeriodReward::get()
		);
		assert_eq!(
			<Test as Config>::Currency::total_issuance(),
			100 * <Test as Config>::InitialPeriodReward::get()
		);
	});
}

#[test]
fn after_initial_period() {
	new_test_ext().execute_with(|| {
		assert!(Balances::free_balance(&TREASURY_ACC).is_zero());
		assert!(<Test as Config>::Currency::total_issuance().is_zero());

		System::set_block_number(<Test as Config>::InitialPeriodLength::get());
		roll_to(<Test as Config>::InitialPeriodLength::get() + 1);
		assert!(Balances::free_balance(&TREASURY_ACC).is_zero());
		assert!(<Test as Config>::Currency::total_issuance().is_zero());

		roll_to(<Test as Config>::InitialPeriodLength::get() + 100);
		assert!(Balances::free_balance(&TREASURY_ACC).is_zero());
		assert!(<Test as Config>::Currency::total_issuance().is_zero());
	});
}
