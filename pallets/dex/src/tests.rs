use crate::liquidity_pool::AssetPair;
use crate::{mock::*, AssetBalanceOf, Config, Error, Event, LiquidityPools, Pallet};
use codec::Compact;
use frame_support::pallet_prelude::*;
use frame_support::{assert_noop, assert_ok};
use sp_runtime::traits::AccountIdConversion;
use std::ops::Sub;

const ASSET_A: u32 = 1u32;
const ASSET_B: u32 = 2u32;
const ADMIN: u64 = 1;
const MINT_BALANCE: u128 = 1;
const ALICE: u64 = 111;
const PALLET: u64 = 9999;
const MIN_LIQUIDITY: u128 = 1000;

type NativeBalance = <Test as crate::Config>::NativeBalance;
type Fungibles = <Test as crate::Config>::Fungibles;

#[test]
fn mint_works() {
	new_test_ext().execute_with(|| {
		let alice = 0;
		let bob = 1;
		let root = RuntimeOrigin::root();
		let origin = RuntimeOrigin::signed(ALICE);
		let initial_amount_a = Dex::expand_to_decimals(1u128);
		let initial_amount_b = Dex::expand_to_decimals(4u128);
		let expected_liquidity = Dex::expand_to_decimals(2u128);
		System::set_block_number(1);

		assert_ok!(Assets::force_create(
			root.clone(),
			Compact::from(ASSET_A),
			ADMIN,
			true,
			MINT_BALANCE
		));
		assert_ok!(Assets::force_create(
			root.clone(),
			Compact::from(ASSET_B),
			ADMIN,
			true,
			MINT_BALANCE
		));
		assert_ok!(Assets::mint(
			RuntimeOrigin::signed(ADMIN),
			Compact::from(ASSET_A),
			ALICE,
			initial_amount_a
		));
		assert_ok!(Assets::mint(
			RuntimeOrigin::signed(ADMIN),
			Compact::from(ASSET_B),
			ALICE,
			initial_amount_b
		));

		assert_ok!(Dex::mint(
			RuntimeOrigin::signed(ALICE),
			ASSET_A,
			ASSET_B,
			initial_amount_a,
			initial_amount_b
		));

		let pool_key = AssetPair::new(ASSET_A, ASSET_B);
		let pool = LiquidityPools::<Test>::get(pool_key).unwrap();

		// Minting of LP Tokens occurred
		assert_eq!(Fungibles::total_supply(pool.id), expected_liquidity);
		assert_eq!(Fungibles::balance(pool.id, ALICE), expected_liquidity.sub(MIN_LIQUIDITY));

		// User balances have been updated
		assert_eq!(Assets::balance(ASSET_A, ALICE), 0);
		assert_eq!(Assets::balance(ASSET_B, ALICE), 0);

		// Pallet manager balances have been updated
		assert_eq!(Assets::balance(ASSET_A, pool.manager), initial_amount_a);
		assert_eq!(Assets::balance(ASSET_B, pool.manager), initial_amount_b);

		// Ensure correct events are triggered
		frame_system::Pallet::<Test>::assert_has_event(RuntimeEvent::Dex(
			Event::LiquidityPoolCreated(ASSET_A, ASSET_B),
		));
		frame_system::Pallet::<Test>::assert_has_event(RuntimeEvent::Dex(Event::LiquidityAdded(
			ASSET_A,
			ASSET_B,
			initial_amount_a,
			initial_amount_b,
		)));
	});
}

#[test]
fn burn_works() {
	new_test_ext().execute_with(|| {
		let alice = 0;
		let bob = 1;
		let root = RuntimeOrigin::root();
		let origin = RuntimeOrigin::signed(ALICE);
		let initial_amount_a = Dex::expand_to_decimals(3u128);
		let initial_amount_b = Dex::expand_to_decimals(3u128);
		let expected_liquidity = Dex::expand_to_decimals(3u128).sub(MIN_LIQUIDITY);
		System::set_block_number(1);

		assert_ok!(Assets::force_create(
			root.clone(),
			Compact::from(ASSET_A),
			ADMIN,
			true,
			MINT_BALANCE
		));
		assert_ok!(Assets::force_create(
			root.clone(),
			Compact::from(ASSET_B),
			ADMIN,
			true,
			MINT_BALANCE
		));
		assert_ok!(Assets::mint(
			RuntimeOrigin::signed(ADMIN),
			Compact::from(ASSET_A),
			ALICE,
			initial_amount_a
		));
		assert_ok!(Assets::mint(
			RuntimeOrigin::signed(ADMIN),
			Compact::from(ASSET_B),
			ALICE,
			initial_amount_b
		));

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

#[test]
fn swapping_token_a_works() {
	new_test_ext().execute_with(|| {
		let initial_amount_a = Dex::expand_to_decimals(50u128);
		let mint_amount_a = Dex::expand_to_decimals(5u128);

		let initial_amount_b = Dex::expand_to_decimals(10u128);
		let swap_amount = Dex::expand_to_decimals(1u128);
		let root = RuntimeOrigin::root();

		assert_ok!(Assets::force_create(
			root.clone(),
			Compact::from(ASSET_A),
			ADMIN,
			true,
			MINT_BALANCE
		));
		assert_ok!(Assets::force_create(
			root.clone(),
			Compact::from(ASSET_B),
			ADMIN,
			true,
			MINT_BALANCE
		));
		assert_ok!(Assets::mint(
			RuntimeOrigin::signed(ADMIN),
			Compact::from(ASSET_A),
			ALICE,
			initial_amount_a
		));
		assert_ok!(Assets::mint(
			RuntimeOrigin::signed(ADMIN),
			Compact::from(ASSET_B),
			ALICE,
			initial_amount_b
		));

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

		assert_ok!(Assets::force_create(
			root.clone(),
			Compact::from(ASSET_A),
			ADMIN,
			true,
			MINT_BALANCE
		));
		assert_ok!(Assets::force_create(
			root.clone(),
			Compact::from(ASSET_B),
			ADMIN,
			true,
			MINT_BALANCE
		));
		assert_ok!(Assets::mint(
			RuntimeOrigin::signed(ADMIN),
			Compact::from(ASSET_A),
			ALICE,
			initial_amount_a
		));
		assert_ok!(Assets::mint(
			RuntimeOrigin::signed(ADMIN),
			Compact::from(ASSET_B),
			ALICE,
			initial_amount_b
		));

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
