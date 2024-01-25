use crate::tests::mock::*;
use crate::types::{AssetPair, LiquidityPool};
use frame_support::{assert_noop, assert_ok};

#[test]
fn swapping_token_a_works() {
	new_test_ext().execute_with(|| {
		let initial_amount_a = Dex::expand_to_decimals(50u128);
		let mint_amount_a = Dex::expand_to_decimals(5u128);
		let initial_amount_b = Dex::expand_to_decimals(10u128);
		let swap_amount = Dex::expand_to_decimals(1u128);
		let root = RuntimeOrigin::root();
		assert_ok!(create_and_mint(root.clone(), ASSET_A, ADMIN, ALICE, initial_amount_a));
		assert_ok!(create_and_mint(root.clone(), ASSET_B, ADMIN, ALICE, initial_amount_b));

		assert_ok!(Dex::mint(
			RuntimeOrigin::signed(ALICE),
			ASSET_A,
			ASSET_B,
			mint_amount_a,
			initial_amount_b
		));

		assert_ok!(Dex::swap(RuntimeOrigin::signed(ALICE), ASSET_A, ASSET_B, swap_amount));

		let pool_key = AssetPair::new(ASSET_A, ASSET_B);
		let pool = LiquidityPools::<Test>::get(pool_key).unwrap();
	});
}

#[test]
fn swapping_token_b_works() {
	new_test_ext().execute_with(|| {
		let initial_amount_a = Dex::expand_to_decimals(50u128);
		let mint_amount_b = Dex::expand_to_decimals(5u128);
		let initial_amount_b = Dex::expand_to_decimals(10u128);
		let swap_amount = Dex::expand_to_decimals(1u128);
		let root = RuntimeOrigin::root();
		assert_ok!(create_and_mint(root.clone(), ASSET_A, ADMIN, ALICE, initial_amount_a));
		assert_ok!(create_and_mint(root.clone(), ASSET_B, ADMIN, ALICE, initial_amount_b));

		assert_ok!(Dex::mint(
			RuntimeOrigin::signed(ALICE),
			ASSET_A,
			ASSET_B,
			initial_amount_a,
			mint_amount_b
		));

		assert_ok!(Dex::swap(RuntimeOrigin::signed(ALICE), ASSET_B, ASSET_A, swap_amount));

		let pool_key = AssetPair::new(ASSET_A, ASSET_B);
		let pool = LiquidityPools::<Test>::get(pool_key).unwrap();
	});
}
