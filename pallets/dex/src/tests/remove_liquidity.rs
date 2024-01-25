use crate::tests::mock::*;
use crate::types::{AssetPair, LiquidityPool};
use crate::{Event, LiquidityPools};
use frame_support::{assert_noop, assert_ok};

#[test]
fn burn_works() {
	let asset_a: AssetId = 1001;
	let asset_b: AssetId = 1002;
	let amount_a: u128 = Dex::expand_to_decimals(3u128);
	let amount_b: u128 = Dex::expand_to_decimals(3u128);

	ExtBuilder::default()
		.with_endowed_balances(vec![(asset_a, ALICE, amount_a), (asset_b, ALICE, amount_b)])
		.build()
		.execute_with(|| {
			let expected_liquidity = Dex::expand_to_decimals(3u128) - MIN_LIQUIDITY;

			assert_ok!(Dex::mint(
				RuntimeOrigin::signed(ALICE),
				asset_a,
				asset_b,
				amount_a,
				amount_b,
			));

			let pool_key = AssetPair::new(asset_a, asset_b);
			let pool = LiquidityPools::<Test>::get(pool_key).unwrap();

			assert_ok!(Dex::burn(
				RuntimeOrigin::signed(ALICE),
				asset_a,
				asset_b,
				expected_liquidity
			));

			// Burning of LP tokens successful
			assert_eq!(Fungibles::balance(pool.id, ALICE), 0);
			assert_eq!(Fungibles::total_supply(pool.id), MIN_LIQUIDITY);
			let asset_a_balance = Fungibles::balance(asset_a, ALICE);
			let asset_b_balance = Fungibles::balance(asset_b, ALICE);

			// Pallet manager balances have been updated
			assert_eq!(Fungibles::balance(asset_a, pool.manager), MIN_LIQUIDITY);
			assert_eq!(Fungibles::balance(asset_b, pool.manager), MIN_LIQUIDITY);

			let token_a_issuance = Fungibles::total_supply(asset_a);
			let token_b_issuance = Fungibles::total_supply(asset_b);

			// User balances have been updated
			assert_eq!(Fungibles::balance(asset_a, ALICE), token_a_issuance - MIN_LIQUIDITY);
			assert_eq!(Fungibles::balance(asset_b, ALICE), token_b_issuance - MIN_LIQUIDITY);

			// Ensure correct events are triggered
			frame_system::Pallet::<Test>::assert_has_event(RuntimeEvent::Dex(
				Event::LiquidityRemoved(asset_a, asset_b, expected_liquidity),
			));
		});
}
