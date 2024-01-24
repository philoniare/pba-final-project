use crate::liquidity_pool::AssetPair;
use crate::{mock::*, AssetBalanceOf, Error, Event, LiquidityPools};
use codec::Compact;
use frame_support::pallet_prelude::*;
use frame_support::{assert_noop, assert_ok};
use sp_runtime::traits::AccountIdConversion;

const ASSET_A: u32 = 1u32;
const ASSET_B: u32 = 2u32;
const ADMIN: u64 = 1;
const MINT_BALANCE: u128 = 1;
const ALICE: u64 = 111;
const PALLET: u64 = 9999;

type NativeBalance = <Test as crate::Config>::NativeBalance;
type Fungibles = <Test as crate::Config>::Fungibles;

#[test]
fn mint_works() {
	new_test_ext().execute_with(|| {
		let alice = 0;
		let bob = 1;
		let root = RuntimeOrigin::root();
		let origin = RuntimeOrigin::signed(ALICE);
		let initial_amount_a = Dex::expand_to_18_decimals(1u128);
		let initial_amount_b = Dex::expand_to_18_decimals(4u128);
		let expected_liquidity = Dex::expand_to_18_decimals(2u128);
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

		let pool_key = AssetPair { asset_a: ASSET_A, asset_b: ASSET_B };
		let pool = LiquidityPools::<Test>::get(pool_key).unwrap();

		// Minting of LP Tokens occurred
		assert_eq!(Fungibles::total_supply(pool.id), expected_liquidity);
		assert_eq!(Fungibles::balance(pool.id, ALICE), expected_liquidity);

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
		let initial_amount_a = Dex::expand_to_18_decimals(3u128);
		let initial_amount_b = Dex::expand_to_18_decimals(3u128);
		let expected_liquidity = Dex::expand_to_18_decimals(3u128);
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

		let pool_key = AssetPair { asset_a: ASSET_A, asset_b: ASSET_B };
		let pool = LiquidityPools::<Test>::get(pool_key).unwrap();

		assert_ok!(Dex::burn(RuntimeOrigin::signed(ALICE), ASSET_A, ASSET_B, expected_liquidity));

		// Burning of LP tokens successful
		assert_eq!(Fungibles::balance(pool.id, ALICE), 0);
		assert_eq!(Fungibles::total_supply(pool.id), 0);

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
