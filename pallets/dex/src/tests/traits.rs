use crate::tests::mock::*;
use crate::traits::{OraclePrice, TokenRatio};
use crate::Error;
use frame_support::{assert_noop, assert_ok};
use sp_runtime::Perbill;
use std::io::ErrorKind::PermissionDenied;

#[test]
fn fetching_token_ratio_works_on_a_to_b() {
	let asset_a: AssetId = 1001;
	let asset_b: AssetId = 1002;
	let amount_a: u128 = expand_to_decimals(10u128);
	let amount_b: u128 = expand_to_decimals(50u128);

	ExtBuilder::default()
		.with_endowed_balances(vec![(asset_a, ALICE, amount_a), (asset_b, ALICE, amount_b)])
		.build()
		.execute_with(|| {
			assert_ok!(Dex::mint(
				RuntimeOrigin::signed(ALICE),
				asset_a,
				asset_b,
				amount_a,
				amount_b,
			));

			assert_eq!(Dex::ratio(asset_a, asset_b), Ok(Perbill::from_percent(20)));
		});
}

#[test]
fn fetching_token_ratio_works_on_b_to_a() {
	let asset_a: AssetId = 1001;
	let asset_b: AssetId = 1002;
	let amount_a: u128 = expand_to_decimals(100u128);
	let amount_b: u128 = expand_to_decimals(50u128);

	ExtBuilder::default()
		.with_endowed_balances(vec![(asset_a, ALICE, amount_a), (asset_b, ALICE, amount_b)])
		.build()
		.execute_with(|| {
			assert_ok!(Dex::mint(
				RuntimeOrigin::signed(ALICE),
				asset_a,
				asset_b,
				amount_a,
				amount_b,
			));

			assert_eq!(Dex::ratio(asset_b, asset_a), Ok(Perbill::from_percent(50)));
		});
}

#[test]
fn fetching_token_ratio_fails_on_identical_assets() {
	let asset_a: AssetId = 1001;
	let asset_b: AssetId = 1002;
	let amount_a: u128 = expand_to_decimals(10u128);
	let amount_b: u128 = expand_to_decimals(50u128);

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
			assert_noop!(Dex::ratio(asset_a, asset_a), Error::<Test>::IdenticalAssets);
		});
}

#[test]
fn fetching_token_ratio_fails_on_nonexistent_pool() {
	let asset_a: AssetId = 1001;
	let asset_b: AssetId = 1002;
	let amount_a: u128 = expand_to_decimals(10u128);
	let amount_b: u128 = expand_to_decimals(50u128);

	ExtBuilder::default()
		.with_endowed_balances(vec![(asset_a, ALICE, amount_a), (asset_b, ALICE, amount_b)])
		.build()
		.execute_with(|| {
			assert_noop!(Dex::ratio(asset_a, asset_b), Error::<Test>::LiquidityPoolDoesNotExist);
		});
}

#[test]
fn fetching_token_ratio_fails_on_unknown_asset_a() {
	let asset_a: AssetId = 1001;
	let asset_b: AssetId = 1002;
	let amount_a: u128 = expand_to_decimals(10u128);
	let amount_b: u128 = expand_to_decimals(50u128);

	ExtBuilder::default()
		.with_endowed_balances(vec![(asset_b, ALICE, amount_b)])
		.build()
		.execute_with(|| {
			assert_noop!(Dex::ratio(asset_a, asset_b), Error::<Test>::UnknownAssetId);
		});
}

#[test]
fn fetching_token_ratio_fails_on_unknown_asset_b() {
	let asset_a: AssetId = 1001;
	let asset_b: AssetId = 1002;
	let amount_a: u128 = expand_to_decimals(10u128);
	let amount_b: u128 = expand_to_decimals(50u128);

	ExtBuilder::default()
		.with_endowed_balances(vec![(asset_a, ALICE, amount_a)])
		.build()
		.execute_with(|| {
			assert_noop!(Dex::ratio(asset_a, asset_b), Error::<Test>::UnknownAssetId);
		});
}

#[test]
fn fetching_price_for_works_from_a_b() {
	let asset_a: AssetId = 1001;
	let asset_b: AssetId = 1002;
	let amount_a: u128 = expand_to_decimals(10u128);
	let amount_b: u128 = expand_to_decimals(50u128);

	ExtBuilder::default()
		.with_endowed_balances(vec![(asset_a, ALICE, amount_a), (asset_b, ALICE, amount_b)])
		.build()
		.execute_with(|| {
			assert_ok!(Dex::mint(
				RuntimeOrigin::signed(ALICE),
				asset_a,
				asset_b,
				amount_a,
				amount_b,
			));

			assert_eq!(Dex::get_price_for(asset_a, 1, asset_b), Ok(4));
		});
}

#[test]
fn fetching_price_for_works_from_b_a() {
	let asset_a: AssetId = 1001;
	let asset_b: AssetId = 1002;
	let amount_a: u128 = expand_to_decimals(300u128);
	let amount_b: u128 = expand_to_decimals(40u128);

	ExtBuilder::default()
		.with_endowed_balances(vec![(asset_a, ALICE, amount_a), (asset_b, ALICE, amount_b)])
		.build()
		.execute_with(|| {
			assert_ok!(Dex::mint(
				RuntimeOrigin::signed(ALICE),
				asset_a,
				asset_b,
				amount_a,
				amount_b,
			));

			assert_eq!(Dex::get_price_for(asset_b, 1, asset_a), Ok(7));
		});
}

#[test]
fn fetching_price_fails_on_identical_assets() {
	let asset_a: AssetId = 1001;
	let asset_b: AssetId = 1002;
	let amount_a: u128 = expand_to_decimals(10u128);
	let amount_b: u128 = expand_to_decimals(50u128);

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
			assert_noop!(Dex::get_price_for(asset_a, 1, asset_a), Error::<Test>::IdenticalAssets);
		});
}

#[test]
fn fetching_price_fails_on_nonexistent_pool() {
	let asset_a: AssetId = 1001;
	let asset_b: AssetId = 1002;
	let amount_a: u128 = expand_to_decimals(10u128);
	let amount_b: u128 = expand_to_decimals(50u128);

	ExtBuilder::default()
		.with_endowed_balances(vec![(asset_a, ALICE, amount_a), (asset_b, ALICE, amount_b)])
		.build()
		.execute_with(|| {
			assert_noop!(
				Dex::get_price_for(asset_a, 1, asset_b),
				Error::<Test>::LiquidityPoolDoesNotExist
			);
		});
}

#[test]
fn fetching_price_fails_on_unknown_asset_a() {
	let asset_a: AssetId = 1001;
	let asset_b: AssetId = 1002;
	let amount_a: u128 = expand_to_decimals(10u128);
	let amount_b: u128 = expand_to_decimals(50u128);

	ExtBuilder::default()
		.with_endowed_balances(vec![(asset_b, ALICE, amount_b)])
		.build()
		.execute_with(|| {
			assert_noop!(Dex::get_price_for(asset_a, 1, asset_b), Error::<Test>::UnknownAssetId);
		});
}

#[test]
fn fetching_price_fails_on_unknown_asset_b() {
	let asset_a: AssetId = 1001;
	let asset_b: AssetId = 1002;
	let amount_a: u128 = expand_to_decimals(10u128);
	let amount_b: u128 = expand_to_decimals(50u128);

	ExtBuilder::default()
		.with_endowed_balances(vec![(asset_a, ALICE, amount_a)])
		.build()
		.execute_with(|| {
			assert_noop!(Dex::get_price_for(asset_a, 1, asset_b), Error::<Test>::UnknownAssetId);
		});
}
