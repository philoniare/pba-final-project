use crate::tests::mock::*;
use frame_support::{assert_noop, assert_ok};
use sp_runtime::Perbill;

#[test]
fn fetching_token_ratio_works() {
	new_test_ext().execute_with(|| {
		let initial_amount_a = Dex::expand_to_decimals(10u128);
		let initial_amount_b = Dex::expand_to_decimals(50u128);
		let root = RuntimeOrigin::root();
		assert_ok!(create_and_mint(root.clone(), ASSET_A, ADMIN, ALICE, initial_amount_a));
		assert_ok!(create_and_mint(root.clone(), ASSET_B, ADMIN, ALICE, initial_amount_b));

		assert_ok!(Dex::mint(
			RuntimeOrigin::signed(ALICE),
			ASSET_A,
			ASSET_B,
			initial_amount_a,
			initial_amount_b,
		));

		assert_eq!(Dex::ratio(ASSET_A, ASSET_B), Ok(Perbill::from_percent(20)));
	});
}

#[test]
fn fetching_price_for_works() {
	new_test_ext().execute_with(|| {
		let initial_amount_a = Dex::expand_to_decimals(10u128);
		let initial_amount_b = Dex::expand_to_decimals(50u128);
		let root = RuntimeOrigin::root();
		assert_ok!(create_and_mint(root.clone(), ASSET_A, ADMIN, ALICE, initial_amount_a));
		assert_ok!(create_and_mint(root.clone(), ASSET_B, ADMIN, ALICE, initial_amount_b));

		assert_ok!(Dex::mint(
			RuntimeOrigin::signed(ALICE),
			ASSET_A,
			ASSET_B,
			initial_amount_a,
			initial_amount_b,
		));

		assert_eq!(Dex::get_price_for(ASSET_A, 1, ASSET_B), Ok(4));
	});
}
