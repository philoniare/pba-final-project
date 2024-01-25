use crate::tests::mock::*;
use crate::types::{AssetPair, LiquidityPool};
use crate::Call::burn;
use crate::{Error, Event, LiquidityPools};
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

#[test]
fn burn_amounts_works_correctly() {
	let asset_a: AssetId = 1001;
	let asset_b: AssetId = 1002;
	let total_a: u128 = Dex::expand_to_decimals(100u128);
	let total_b: u128 = Dex::expand_to_decimals(100u128);
	let amount_a: u128 = Dex::expand_to_decimals(10u128);
	let amount_b: u128 = Dex::expand_to_decimals(40u128);
	let second_amount_a: u128 = Dex::expand_to_decimals(50u128);
	let second_amount_b: u128 = Dex::expand_to_decimals(10u128);

	ExtBuilder::default()
		.with_endowed_balances(vec![(asset_a, ALICE, total_a), (asset_b, ALICE, total_b)])
		.build()
		.execute_with(|| {
			let expected_liquidity = Dex::expand_to_decimals(25u128);
			assert_ok!(Dex::mint(
				RuntimeOrigin::signed(ALICE.into()),
				asset_a,
				asset_b,
				amount_a,
				amount_b
			));
			let pool_key = AssetPair { asset_a, asset_b };
			let pool = LiquidityPools::<Test>::get(pool_key).unwrap();
			assert_eq!(Fungibles::total_supply(pool.id), Dex::expand_to_decimals(20u128));

			assert_ok!(Dex::mint(
				RuntimeOrigin::signed(ALICE.into()),
				asset_a,
				asset_b,
				second_amount_a,
				second_amount_b
			));

			let burn_amount = Dex::expand_to_decimals(1u128);
			assert_ok!(Dex::burn(RuntimeOrigin::signed(ALICE), asset_a, asset_b, burn_amount));

			// Burning of LP tokens successful
			assert_eq!(
				Fungibles::balance(pool.id, ALICE),
				expected_liquidity - burn_amount - MIN_LIQUIDITY
			);
			assert_eq!(Fungibles::total_supply(pool.id), expected_liquidity - burn_amount);
			let asset_a_balance = Fungibles::balance(asset_a, ALICE);
			let asset_b_balance = Fungibles::balance(asset_b, ALICE);

			// Pallet manager balances have been updated
			assert_eq!(Fungibles::balance(asset_a, pool.manager), 576000000000);
			assert_eq!(Fungibles::balance(asset_b, pool.manager), 480000000000);

			let token_a_issuance = Fungibles::total_supply(asset_a);
			let token_b_issuance = Fungibles::total_supply(asset_b);

			// User balances have been updated
			assert_eq!(Fungibles::balance(asset_a, ALICE), 424000000000);
			assert_eq!(Fungibles::balance(asset_b, ALICE), 520000000000);

			// Ensure correct events are triggered
			frame_system::Pallet::<Test>::assert_has_event(RuntimeEvent::Dex(
				Event::LiquidityRemoved(asset_a, asset_b, burn_amount),
			));
		});
}

// #[test]
// fn burn_fails_with() {
// 	let asset_a: AssetId = 1001;
// 	let asset_b: AssetId = 1002;
// 	let amount_a: u128 = Dex::expand_to_decimals(1u128);
// 	let amount_b: u128 = Dex::expand_to_decimals(4u128);
//
// 	ExtBuilder::default()
// 		.with_endowed_balances(vec![(asset_a, ALICE, amount_a), (asset_b, ALICE, amount_b)])
// 		.build()
// 		.execute_with(|| {
// 			let expected_liquidity = Dex::expand_to_decimals(2u128);
// 			assert_noop!(
// 				Dex::mint(RuntimeOrigin::signed(ALICE.into()), asset_a, asset_b, amount_a, 0),
// 				Error::<Test>::InsufficientInputAmount
// 			);
// 		});
// }