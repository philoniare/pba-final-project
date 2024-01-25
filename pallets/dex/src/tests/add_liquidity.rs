use crate::tests::mock::*;
use crate::types::{AssetPair, LiquidityPool};
use crate::{Event, LiquidityPools};
use frame_support::{assert_noop, assert_ok};

#[test]
fn mint_works() {
	let asset_a: AssetId = 1001;
	let asset_b: AssetId = 1002;
	let amount_a: u128 = Dex::expand_to_decimals(1u128);
	let amount_b: u128 = Dex::expand_to_decimals(4u128);

	ExtBuilder::default()
		.with_endowed_balances(vec![(asset_a, ALICE, amount_a), (asset_b, ALICE, amount_b)])
		.build()
		.execute_with(|| {
			let expected_liquidity = Dex::expand_to_decimals(2u128);
			assert_ok!(Dex::mint(
				RuntimeOrigin::signed(ALICE.into()),
				asset_a,
				asset_b,
				amount_a,
				amount_b
			));

			let pool_key = AssetPair::new(asset_a, asset_b);
			let pool = LiquidityPools::<Test>::get(pool_key).unwrap();

			// Minting of LP Tokens occurred
			assert_eq!(Fungibles::total_supply(pool.id), expected_liquidity);
			assert_eq!(Fungibles::balance(pool.id, ALICE), expected_liquidity - MIN_LIQUIDITY);

			// User balances have been updated
			assert_eq!(Assets::balance(asset_a, ALICE), 0);
			assert_eq!(Assets::balance(asset_b, ALICE), 0);

			// Pallet manager balances have been updated
			assert_eq!(Assets::balance(asset_a, pool.manager), amount_a);
			assert_eq!(Assets::balance(asset_b, pool.manager), amount_b);

			// Ensure correct events are triggered
			frame_system::Pallet::<Test>::assert_has_event(RuntimeEvent::Dex(
				Event::LiquidityPoolCreated(asset_a, asset_b),
			));
			frame_system::Pallet::<Test>::assert_has_event(RuntimeEvent::Dex(
				Event::LiquidityAdded(asset_a, asset_b, amount_a, amount_b),
			));
		});
}
