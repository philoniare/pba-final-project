//! Benchmarking setup for pallet-dex
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as Dex;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

#[benchmarks]
mod benchmarks {
	use super::*;
	use crate::types::AssetPair;

	#[benchmark]
	fn mint() {
		let asset_a: AssetIdOf<T> = 1u32;
		let asset_b: AssetIdOf<T> = 2u32;
		let pool_id: AssetIdOf<T> = 100u32;
		let amount_a: AssetBalanceOf<T> = 100000u32.into();
		let amount_b: AssetBalanceOf<T> = 200000u32.into();
		let caller: T::AccountId = whitelisted_caller();
		let _ =
			T::Fungibles::create(asset_a.clone(), caller.clone(), true, AssetBalanceOf::<T>::one());
		let _ = T::Fungibles::mint_into(asset_a.clone(), &caller, 1_000_000_000u32.into());
		let _ =
			T::Fungibles::create(asset_b.clone(), caller.clone(), true, AssetBalanceOf::<T>::one());
		let _ = T::Fungibles::mint_into(asset_b.clone(), &caller, 1_000_000_000u32.into());

		#[extrinsic_call]
		_(RawOrigin::Signed(caller.clone()), pool_id, asset_a, asset_b, amount_a, amount_b);

		let pool_key = AssetPair::new(asset_a.clone(), asset_b.clone());
		// Panics if pool does not exist
		let pool = LiquidityPools::<T>::get(pool_key).unwrap();

		let pool_asset_a_balance = T::Fungibles::balance(asset_a.clone(), &pool.manager);
		assert_eq!(pool_asset_a_balance, 100_000u32.into());

		let pool_asset_b_balance = T::Fungibles::balance(asset_b.clone(), &pool.manager);
		assert_eq!(pool_asset_b_balance, 200_000u32.into());

		let caller_asset_a_balance = T::Fungibles::balance(asset_a.clone(), &caller);
		assert_eq!(caller_asset_a_balance, 999_900_000u32.into());

		let caller_asset_b_balance = T::Fungibles::balance(asset_b.clone(), &caller);
		assert_eq!(caller_asset_b_balance, 999_800_000u32.into());
	}

	impl_benchmark_test_suite!(
		Dex,
		crate::tests::mock::ExtBuilder::default().build(),
		crate::tests::mock::Test
	);
}
