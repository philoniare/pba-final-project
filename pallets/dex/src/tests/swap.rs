use crate::tests::mock::*;
use crate::types::{AssetPair, LiquidityPool};
use crate::{Event, LiquidityPools};
use frame_support::{assert_noop, assert_ok};

#[test]
fn swapping_token_a_works() {
	let asset_a: AssetId = 1001;
	let asset_b: AssetId = 1002;
	let amount_a: u128 = Dex::expand_to_decimals(50u128);
	let amount_b: u128 = Dex::expand_to_decimals(10u128);

	ExtBuilder::default()
		.with_endowed_balances(vec![(asset_a, ALICE, amount_a), (asset_b, ALICE, amount_b)])
		.build()
		.execute_with(|| {
			let mint_amount_a = Dex::expand_to_decimals(5u128);
			let swap_amount = Dex::expand_to_decimals(1u128);
			let root = RuntimeOrigin::root();

			assert_ok!(Dex::mint(
				RuntimeOrigin::signed(ALICE),
				asset_a,
				asset_b,
				mint_amount_a,
				amount_b
			));

			assert_ok!(Dex::swap(RuntimeOrigin::signed(ALICE), asset_a, asset_b, swap_amount));

			let pool_key = AssetPair::new(asset_a, asset_b);
			let pool = LiquidityPools::<Test>::get(pool_key).unwrap();
		});
}

#[test]
fn swapping_token_b_works() {
	let asset_a: AssetId = 1001;
	let asset_b: AssetId = 1002;
	let amount_a: u128 = Dex::expand_to_decimals(50u128);
	let amount_b: u128 = Dex::expand_to_decimals(10u128);
	ExtBuilder::default()
		.with_endowed_balances(vec![(asset_a, ALICE, amount_a), (asset_b, ALICE, amount_b)])
		.build()
		.execute_with(|| {
			let mint_amount_b = Dex::expand_to_decimals(5u128);
			let swap_amount = Dex::expand_to_decimals(1u128);
			let root = RuntimeOrigin::root();

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
		});
}
