use crate::tests::mock::*;
use crate::types::{AssetPair, LiquidityPool};
use frame_support::{assert_noop, assert_ok};

#[test]
fn burn_works() {
	new_test_ext().execute_with(|| {
		let root = RuntimeOrigin::root();
		let initial_amount_a = Dex::expand_to_decimals(3u128);
		let initial_amount_b = Dex::expand_to_decimals(3u128);
		let expected_liquidity = Dex::expand_to_decimals(3u128).sub(MIN_LIQUIDITY);
		System::set_block_number(1);
		assert_ok!(create_and_mint(root.clone(), ASSET_A, ADMIN, ALICE, initial_amount_a));
		assert_ok!(create_and_mint(root.clone(), ASSET_B, ADMIN, ALICE, initial_amount_b));

		assert_ok!(Dex::mint(
			RuntimeOrigin::signed(ALICE),
			ASSET_A,
			ASSET_B,
			initial_amount_a,
			initial_amount_b
		));

		let pool_key = AssetPair::new(ASSET_A, ASSET_B);
		let pool = LiquidityPools::<Test>::get(pool_key).unwrap();

		assert_ok!(Dex::burn(RuntimeOrigin::signed(ALICE), ASSET_A, ASSET_B, expected_liquidity));

		// Burning of LP tokens successful
		assert_eq!(Fungibles::balance(pool.id, ALICE), 0);
		assert_eq!(Fungibles::total_supply(pool.id), MIN_LIQUIDITY);
		let asset_a_balance = Assets::balance(ASSET_A, ALICE);
		let asset_b_balance = Assets::balance(ASSET_B, ALICE);

		// Pallet manager balances have been updated
		assert_eq!(Assets::balance(ASSET_A, pool.manager), MIN_LIQUIDITY);
		assert_eq!(Assets::balance(ASSET_B, pool.manager), MIN_LIQUIDITY);

		let token_a_issuance = Fungibles::total_supply(ASSET_A);
		let token_b_issuance = Fungibles::total_supply(ASSET_B);

		// User balances have been updated
		assert_eq!(Assets::balance(ASSET_A, ALICE), token_a_issuance.sub(MIN_LIQUIDITY));
		assert_eq!(Assets::balance(ASSET_B, ALICE), token_b_issuance.sub(MIN_LIQUIDITY));

		// Ensure correct events are triggered
		frame_system::Pallet::<Test>::assert_has_event(RuntimeEvent::Dex(Event::LiquidityRemoved(
			ASSET_A,
			ASSET_B,
			expected_liquidity,
		)));
	});
}
