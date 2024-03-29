use crate::tests::mock::*;
use crate::types::AssetPair;
use crate::{Error, Event, LiquidityPools};
use frame_support::{assert_noop, assert_ok};

#[test]
fn mint_works() {
	let asset_a: AssetId = 1001;
	let asset_b: AssetId = 1002;
	let pool_id: AssetId = 10000;
	let amount_a: u128 = expand_to_decimals(1u128);
	let amount_b: u128 = expand_to_decimals(4u128);

	ExtBuilder::default()
		.with_endowed_balances(vec![(asset_a, ALICE, amount_a), (asset_b, ALICE, amount_b)])
		.build()
		.execute_with(|| {
			let expected_liquidity = expand_to_decimals(2u128);
			assert_ok!(Dex::mint(
				RuntimeOrigin::signed(ALICE.into()),
				pool_id,
				asset_a,
				asset_b,
				amount_a,
				amount_b
			));

			let pool_key = AssetPair::new(asset_a, asset_b);
			let pool = LiquidityPools::<Test>::get(pool_key).unwrap();

			// Internal balances should be updated
			assert_eq!(pool.asset_a_balance, amount_a);
			assert_eq!(pool.asset_b_balance, amount_b);

			// Minting of LP Tokens occurred
			assert_eq!(Fungibles::total_supply(pool.id), expected_liquidity);
			assert_eq!(Fungibles::balance(pool.id, ALICE), expected_liquidity - MIN_LIQUIDITY);

			// User balances have been updated
			assert_eq!(Fungibles::balance(asset_a, ALICE), 0);
			assert_eq!(Fungibles::balance(asset_b, ALICE), 0);

			// Pallet manager balances have been updated
			assert_eq!(Fungibles::balance(asset_a, pool.manager), amount_a);
			assert_eq!(Fungibles::balance(asset_b, pool.manager), amount_b);

			// Ensure correct events are triggered
			frame_system::Pallet::<Test>::assert_has_event(RuntimeEvent::Dex(
				Event::LiquidityPoolCreated(pool_id, asset_a, asset_b),
			));
			frame_system::Pallet::<Test>::assert_has_event(RuntimeEvent::Dex(
				Event::LiquidityAdded(asset_a, asset_b, amount_a, amount_b),
			));
		});
}

#[test]
fn mint_works_with_same_asset_in_multiple_pools() {
	let asset_a: AssetId = 1001;
	let asset_b: AssetId = 1002;
	let asset_c: AssetId = 1003;
	let pool_id: AssetId = 10000;
	let pool_id_2: AssetId = 10001;
	let total: u128 = expand_to_decimals(100u128);
	let amount_a: u128 = expand_to_decimals(1u128);
	let amount_b: u128 = expand_to_decimals(4u128);
	let amount_c: u128 = expand_to_decimals(10u128);
	let burn_amount: u128 = expand_to_decimals(1u128);

	ExtBuilder::default()
		.with_endowed_balances(vec![
			(asset_a, ALICE, total),
			(asset_b, ALICE, total),
			(asset_c, ALICE, total),
		])
		.build()
		.execute_with(|| {
			// Create pool for A - B
			assert_ok!(Dex::mint(
				RuntimeOrigin::signed(ALICE.into()),
				pool_id,
				asset_a,
				asset_b,
				amount_a,
				amount_b
			));

			// Create pool for A - C
			assert_ok!(Dex::mint(
				RuntimeOrigin::signed(ALICE.into()),
				pool_id_2,
				asset_a,
				asset_c,
				amount_a,
				amount_c
			));

			assert_ok!(Dex::burn(
				RuntimeOrigin::signed(ALICE.into()),
				asset_a,
				asset_c,
				burn_amount
			));

			// Removing from one pool shouldn't affect the reserves in another
			let first_pool_key = AssetPair::new(asset_a, asset_b);
			let first_pool = LiquidityPools::<Test>::get(first_pool_key).unwrap();

			let second_pool_key = AssetPair::new(asset_a, asset_c);
			let second_pool = LiquidityPools::<Test>::get(second_pool_key).unwrap();

			// Balances in the first pool should not be affected
			assert_eq!(first_pool.asset_a_balance, amount_a);
			assert_eq!(first_pool.asset_b_balance, amount_b);

			// Liquidity should be removed from second pool
			assert_eq!(second_pool.asset_a_balance, 6837722340);
			assert_eq!(second_pool.asset_b_balance, 68377223398);

			// Pallet manager balances have been updated
			assert_eq!(Fungibles::balance(asset_a, first_pool.manager), 16837722340);
			assert_eq!(Fungibles::balance(asset_b, first_pool.manager), amount_b);
		});
}

#[test]
fn mint_works_increments_counter_on_multiple_pools() {
	let asset_a: AssetId = 1001;
	let asset_b: AssetId = 1002;
	let asset_c: AssetId = 1003;
	let pool_id: AssetId = 10000;
	let pool_id_2: AssetId = 10001;
	let total: u128 = expand_to_decimals(10u128);
	let amount_a: u128 = expand_to_decimals(1u128);
	let amount_b: u128 = expand_to_decimals(4u128);

	ExtBuilder::default()
		.with_endowed_balances(vec![
			(asset_a, ALICE, total),
			(asset_b, ALICE, total),
			(asset_c, ALICE, total),
		])
		.build()
		.execute_with(|| {
			let expected_liquidity = expand_to_decimals(2u128);
			// Create the first pool
			assert_ok!(Dex::mint(
				RuntimeOrigin::signed(ALICE.into()),
				pool_id,
				asset_a,
				asset_b,
				amount_a,
				amount_b
			));

			assert_ok!(Dex::mint(
				RuntimeOrigin::signed(ALICE.into()),
				pool_id_2,
				asset_a,
				asset_c,
				amount_a,
				amount_b
			));

			let pool_key = AssetPair::new(asset_a, asset_c);
			let pool = LiquidityPools::<Test>::get(pool_key).unwrap();

			// Minting of LP Tokens occurred
			assert_eq!(Fungibles::total_supply(pool.id), expected_liquidity);
			assert_eq!(Fungibles::balance(pool.id, ALICE), expected_liquidity - MIN_LIQUIDITY);
			assert_eq!(pool.id, pool_id_2);
		});
}

#[test]
fn mint_works_with_existing_pool() {
	let asset_a: AssetId = 1001;
	let asset_b: AssetId = 1002;
	let pool_id: AssetId = 10000;
	let pool_id_2: AssetId = 10001;
	let total_a: u128 = expand_to_decimals(100u128);
	let total_b: u128 = expand_to_decimals(100u128);
	let amount_a: u128 = expand_to_decimals(10u128);
	let amount_b: u128 = expand_to_decimals(40u128);
	let second_amount_a: u128 = expand_to_decimals(50u128);
	let second_amount_b: u128 = expand_to_decimals(10u128);

	ExtBuilder::default()
		.with_endowed_balances(vec![(asset_a, ALICE, total_a), (asset_b, ALICE, total_b)])
		.build()
		.execute_with(|| {
			let expected_liquidity = expand_to_decimals(25u128);
			assert_ok!(Dex::mint(
				RuntimeOrigin::signed(ALICE.into()),
				pool_id,
				asset_a,
				asset_b,
				amount_a,
				amount_b
			));
			let pool_key = AssetPair { asset_a, asset_b };
			let pool = LiquidityPools::<Test>::get(pool_key).unwrap();
			assert_eq!(Fungibles::total_supply(pool.id), expand_to_decimals(20u128));

			assert_ok!(Dex::mint(
				RuntimeOrigin::signed(ALICE.into()),
				pool_id_2,
				asset_a,
				asset_b,
				second_amount_a,
				second_amount_b
			));

			// Minting of LP Tokens occurred
			assert_eq!(Fungibles::total_supply(pool.id), expected_liquidity);
			assert_eq!(Fungibles::balance(pool.id, ALICE), expected_liquidity - MIN_LIQUIDITY);

			// User balances have been updated
			assert_eq!(Fungibles::balance(asset_a, ALICE), total_a - amount_a - second_amount_a);
			assert_eq!(Fungibles::balance(asset_b, ALICE), total_b - amount_b - second_amount_b);

			// Pallet manager balances have been updated
			assert_eq!(Fungibles::balance(asset_a, pool.manager), amount_a + second_amount_a);
			assert_eq!(Fungibles::balance(asset_b, pool.manager), amount_b + second_amount_b);

			// Ensure correct events are triggered
			frame_system::Pallet::<Test>::assert_has_event(RuntimeEvent::Dex(
				Event::LiquidityPoolCreated(pool_id, asset_a, asset_b),
			));
			frame_system::Pallet::<Test>::assert_has_event(RuntimeEvent::Dex(
				Event::LiquidityAdded(asset_a, asset_b, second_amount_a, second_amount_b),
			));
		});
}

#[test]
fn mint_sorting_works() {
	let asset_a: AssetId = 1001;
	let asset_b: AssetId = 1002;
	let pool_id: AssetId = 10000;
	let amount_a: u128 = expand_to_decimals(1u128);
	let amount_b: u128 = expand_to_decimals(4u128);

	ExtBuilder::default()
		.with_endowed_balances(vec![(asset_a, ALICE, amount_a), (asset_b, ALICE, amount_b)])
		.build()
		.execute_with(|| {
			let expected_liquidity = expand_to_decimals(2u128);
			assert_ok!(Dex::mint(
				RuntimeOrigin::signed(ALICE.into()),
				pool_id,
				asset_b,
				asset_a,
				amount_a,
				amount_b
			));

			let pool_key = AssetPair { asset_a, asset_b };
			let pool = LiquidityPools::<Test>::get(pool_key).unwrap();

			// Minting of LP Tokens occurred
			assert_eq!(Fungibles::total_supply(pool.id), expected_liquidity);
			assert_eq!(Fungibles::balance(pool.id, ALICE), expected_liquidity - MIN_LIQUIDITY);

			// User balances have been updated
			assert_eq!(Fungibles::balance(asset_a, ALICE), 0);
			assert_eq!(Fungibles::balance(asset_b, ALICE), 0);

			// Pallet manager balances have been updated
			assert_eq!(Fungibles::balance(asset_a, pool.manager), amount_a);
			assert_eq!(Fungibles::balance(asset_b, pool.manager), amount_b);

			// Ensure correct events are triggered
			frame_system::Pallet::<Test>::assert_has_event(RuntimeEvent::Dex(
				Event::LiquidityPoolCreated(pool_id, asset_a, asset_b),
			));
			frame_system::Pallet::<Test>::assert_has_event(RuntimeEvent::Dex(
				Event::LiquidityAdded(asset_a, asset_b, amount_a, amount_b),
			));
		});
}

#[test]
fn mint_fails_with_invalid_assets() {
	let asset_a: AssetId = 1001;
	let asset_b: AssetId = 1002;
	let pool_id: AssetId = 10000;
	let amount_a: u128 = expand_to_decimals(1u128);
	let amount_b: u128 = expand_to_decimals(4u128);

	ExtBuilder::default()
		.with_endowed_balances(vec![(asset_a, ALICE, amount_a), (asset_b, ALICE, amount_b)])
		.build()
		.execute_with(|| {
			assert_noop!(
				Dex::mint(
					RuntimeOrigin::signed(ALICE.into()),
					pool_id,
					asset_a,
					asset_a,
					amount_a,
					amount_b
				),
				Error::<Test>::IdenticalAssets
			);
		});
}

#[test]
fn mint_fails_with_token_a_0_amount() {
	let asset_a: AssetId = 1001;
	let asset_b: AssetId = 1002;
	let pool_id: AssetId = 10000;
	let amount_a: u128 = expand_to_decimals(1u128);
	let amount_b: u128 = expand_to_decimals(4u128);

	ExtBuilder::default()
		.with_endowed_balances(vec![(asset_a, ALICE, amount_a), (asset_b, ALICE, amount_b)])
		.build()
		.execute_with(|| {
			assert_noop!(
				Dex::mint(
					RuntimeOrigin::signed(ALICE.into()),
					pool_id,
					asset_a,
					asset_b,
					0,
					amount_b
				),
				Error::<Test>::InsufficientInputAmount
			);
		});
}

#[test]
fn mint_fails_with_token_b_0_amount() {
	let asset_a: AssetId = 1001;
	let asset_b: AssetId = 1002;
	let pool_id: AssetId = 10000;
	let amount_a: u128 = expand_to_decimals(1u128);
	let amount_b: u128 = expand_to_decimals(4u128);

	ExtBuilder::default()
		.with_endowed_balances(vec![(asset_a, ALICE, amount_a), (asset_b, ALICE, amount_b)])
		.build()
		.execute_with(|| {
			assert_noop!(
				Dex::mint(
					RuntimeOrigin::signed(ALICE.into()),
					pool_id,
					asset_a,
					asset_b,
					amount_a,
					0
				),
				Error::<Test>::InsufficientInputAmount
			);
		});
}

#[test]
fn mint_fails_with_insufficient_liquidity() {
	let asset_a: AssetId = 1001;
	let asset_b: AssetId = 1002;
	let pool_id: AssetId = 10000;
	let amount_a: u128 = 1;
	let amount_b: u128 = 4;

	ExtBuilder::default()
		.with_endowed_balances(vec![(asset_a, ALICE, amount_a), (asset_b, ALICE, amount_b)])
		.build()
		.execute_with(|| {
			assert_noop!(
				Dex::mint(
					RuntimeOrigin::signed(ALICE.into()),
					pool_id,
					asset_a,
					asset_b,
					amount_a,
					amount_b
				),
				Error::<Test>::InsufficientLiquidity
			);
		});
}

#[test]
fn mint_fails_with_unknown_asset_id_a() {
	let asset_a: AssetId = 1001;
	let asset_b: AssetId = 1002;
	let pool_id: AssetId = 10000;
	let amount_a: u128 = 1;
	let amount_b: u128 = 4;

	ExtBuilder::default()
		.with_endowed_balances(vec![(asset_a, ALICE, amount_a)])
		.build()
		.execute_with(|| {
			assert_noop!(
				Dex::mint(
					RuntimeOrigin::signed(ALICE.into()),
					pool_id,
					asset_a,
					asset_b,
					amount_a,
					amount_b
				),
				Error::<Test>::UnknownAssetId
			);
		});
}

#[test]
fn mint_fails_with_unknown_asset_id_b() {
	let asset_a: AssetId = 1001;
	let asset_b: AssetId = 1002;
	let pool_id: AssetId = 10000;
	let amount_a: u128 = 1;
	let amount_b: u128 = 4;

	ExtBuilder::default()
		.with_endowed_balances(vec![(asset_b, ALICE, amount_b)])
		.build()
		.execute_with(|| {
			assert_noop!(
				Dex::mint(
					RuntimeOrigin::signed(ALICE.into()),
					pool_id,
					asset_a,
					asset_b,
					amount_a,
					amount_b
				),
				Error::<Test>::UnknownAssetId
			);
		});
}
