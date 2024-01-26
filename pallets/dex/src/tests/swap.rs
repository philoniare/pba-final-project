use crate::tests::mock::*;
use crate::types::AssetPair;
use crate::{Error, Event, LiquidityPools};
use frame_support::{assert_noop, assert_ok};

#[test]
fn swapping_token_a_works() {
	let asset_a: AssetId = 1001;
	let asset_b: AssetId = 1002;
	let total_a: u128 = expand_to_decimals(100u128);
	let total_b: u128 = expand_to_decimals(100u128);
	let amount_a: u128 = expand_to_decimals(10u128);
	let amount_b: u128 = expand_to_decimals(10u128);

	ExtBuilder::default()
		.with_endowed_balances(vec![(asset_a, ALICE, total_a), (asset_b, ALICE, total_b)])
		.build()
		.execute_with(|| {
			let swap_amount = expand_to_decimals(1u128);
			assert_ok!(Dex::mint(
				RuntimeOrigin::signed(ALICE),
				asset_a,
				asset_b,
				amount_a,
				amount_b
			));
			assert_ok!(Dex::swap(RuntimeOrigin::signed(ALICE), asset_a, asset_b, swap_amount));

			let pool_key = AssetPair::new(asset_a, asset_b);
			let pool = LiquidityPools::<Test>::get(pool_key).unwrap();

			assert_eq!(Fungibles::balance(asset_a, pool.manager), expand_to_decimals(11u128));
			let pool_asset_b = decimals_to_numeric(Fungibles::balance(asset_b, pool.manager));
			assert_eq!(pool_asset_b, 9u128);

			let alice_asset_a = decimals_to_numeric(Fungibles::balance(asset_a, ALICE));
			assert_eq!(alice_asset_a, 89u128);
			let alice_asset_b = decimals_to_numeric(Fungibles::balance(asset_b, ALICE));
			assert_eq!(alice_asset_b, 91u128);
		});
}

#[test]
fn swapping_token_b_works() {
	let asset_a: AssetId = 1001;
	let asset_b: AssetId = 1002;
	let amount_a: u128 = expand_to_decimals(50u128);
	let amount_b: u128 = expand_to_decimals(10u128);
	ExtBuilder::default()
		.with_endowed_balances(vec![(asset_a, ALICE, amount_a), (asset_b, ALICE, amount_b)])
		.build()
		.execute_with(|| {
			let mint_amount_b = expand_to_decimals(5u128);
			let swap_amount = expand_to_decimals(1u128);

			assert_ok!(Dex::mint(
				RuntimeOrigin::signed(ALICE),
				asset_a,
				asset_b,
				amount_b,
				mint_amount_b
			));

			assert_ok!(Dex::swap(RuntimeOrigin::signed(ALICE), asset_b, asset_a, swap_amount));

			let pool_key = AssetPair::new(asset_a, asset_b);
			let pool = LiquidityPools::<Test>::get(pool_key).unwrap();

			let pool_asset_a = decimals_to_numeric(Fungibles::balance(asset_a, pool.manager));
			assert_eq!(pool_asset_a, 8u128);
			let pool_asset_b = decimals_to_numeric(Fungibles::balance(asset_b, pool.manager));
			assert_eq!(pool_asset_b, 6u128);
			//
			let alice_asset_a = decimals_to_numeric(Fungibles::balance(asset_a, ALICE));
			assert_eq!(alice_asset_a, 42u128);
			let alice_asset_b = decimals_to_numeric(Fungibles::balance(asset_b, ALICE));
			assert_eq!(alice_asset_b, 4);
		});
}

#[test]
fn swapping_fails_on_non_existing_pool() {
	let asset_a: AssetId = 1001;
	let asset_b: AssetId = 1002;
	let amount_a: u128 = expand_to_decimals(50u128);
	let amount_b: u128 = expand_to_decimals(10u128);
	ExtBuilder::default()
		.with_endowed_balances(vec![(asset_a, ALICE, amount_a), (asset_b, ALICE, amount_b)])
		.build()
		.execute_with(|| {
			let swap_amount = expand_to_decimals(1u128);
			assert_noop!(
				Dex::swap(RuntimeOrigin::signed(ALICE), asset_a, asset_b, swap_amount),
				Error::<Test>::LiquidityPoolDoesNotExist
			);
		});
}

#[test]
fn swapping_fails_on_idential_assets() {
	let asset_a: AssetId = 1001;
	let asset_b: AssetId = 1002;
	let amount_a: u128 = expand_to_decimals(50u128);
	let amount_b: u128 = expand_to_decimals(10u128);
	ExtBuilder::default()
		.with_endowed_balances(vec![(asset_a, ALICE, amount_a), (asset_b, ALICE, amount_b)])
		.build()
		.execute_with(|| {
			let swap_amount = expand_to_decimals(1u128);
			assert_noop!(
				Dex::swap(RuntimeOrigin::signed(ALICE), asset_a, asset_a, swap_amount),
				Error::<Test>::IdenticalAssets
			);
		});
}

#[test]
fn swapping_fails_on_zero_amount_in() {
	let asset_a: AssetId = 1001;
	let asset_b: AssetId = 1002;
	let amount_a: u128 = expand_to_decimals(50u128);
	let amount_b: u128 = expand_to_decimals(10u128);
	ExtBuilder::default()
		.with_endowed_balances(vec![(asset_a, ALICE, amount_a), (asset_b, ALICE, amount_b)])
		.build()
		.execute_with(|| {
			assert_ok!(Dex::mint(
				RuntimeOrigin::signed(ALICE),
				asset_a,
				asset_b,
				amount_a,
				amount_b
			));
			let swap_amount = expand_to_decimals(1u128);
			assert_noop!(
				Dex::swap(RuntimeOrigin::signed(ALICE), asset_a, asset_b, 0),
				Error::<Test>::InsufficientInputAmount
			);
		});
}

#[test]
fn swapping_fails_on_greater_than_pool_amount() {
	let asset_a: AssetId = 1001;
	let asset_b: AssetId = 1002;
	let amount_a: u128 = expand_to_decimals(50u128);
	let amount_b: u128 = expand_to_decimals(10u128);
	ExtBuilder::default()
		.with_endowed_balances(vec![(asset_a, ALICE, amount_a), (asset_b, ALICE, amount_b)])
		.build()
		.execute_with(|| {
			assert_ok!(Dex::mint(
				RuntimeOrigin::signed(ALICE),
				asset_a,
				asset_b,
				amount_a,
				amount_b
			));
			let swap_amount = expand_to_decimals(60u128);
			assert_noop!(
				Dex::swap(RuntimeOrigin::signed(ALICE), asset_a, asset_b, swap_amount),
				Error::<Test>::InsufficientLiquidity
			);
		});
}
