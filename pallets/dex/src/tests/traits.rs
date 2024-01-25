use crate::tests::mock::*;
use crate::traits::{OraclePrice, TokenRatio};
use frame_support::{assert_noop, assert_ok};
use sp_runtime::Perbill;

#[test]
fn fetching_token_ratio_works() {
	let asset_a: AssetId = 1001;
	let asset_b: AssetId = 1002;
	let amount_a: u128 = Dex::expand_to_decimals(10u128);
	let amount_b: u128 = Dex::expand_to_decimals(50u128);

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
fn fetching_price_for_works() {
	let asset_a: AssetId = 1001;
	let asset_b: AssetId = 1002;
	let amount_a: u128 = Dex::expand_to_decimals(10u128);
	let amount_b: u128 = Dex::expand_to_decimals(50u128);

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

// TODO: Handle case where it's in reverse direction
