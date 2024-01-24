#![cfg_attr(not(feature = "std"), no_std)]

use crate::liquidity_pool::LiquidityPool;
use frame_support::sp_runtime::traits::{One, Zero};
use frame_support::traits::fungibles;
use frame_support::PalletId;
pub use pallet::*;

mod liquidity_pool;
#[cfg(test)]
mod mock;
mod util;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

use frame_support::traits::fungible;
use frame_support::traits::fungibles::*;

pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
pub type AssetIdOf<T> = <<T as Config>::Fungibles as fungibles::Inspect<
	<T as frame_system::Config>::AccountId,
>>::AssetId;

pub type BalanceOf<T> = <<T as Config>::NativeBalance as fungible::Inspect<
	<T as frame_system::Config>::AccountId,
>>::Balance;

pub type AssetBalanceOf<T> = <<T as Config>::Fungibles as fungibles::Inspect<
	<T as frame_system::Config>::AccountId,
>>::Balance;

#[frame_support::pallet]
pub mod pallet {
	use crate::liquidity_pool::AssetPair;
	use crate::*;
	use frame_support::{
		pallet_prelude::*,
		traits::{
			fungible::{self, *},
			fungibles::{self, *},
		},
	};
	use frame_system::pallet_prelude::*;
	use sp_runtime::traits::AccountIdConversion;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Type to access the Balances Pallet.
		type NativeBalance: fungible::Inspect<Self::AccountId>
			+ fungible::Mutate<Self::AccountId>
			+ fungible::hold::Inspect<Self::AccountId>
			+ fungible::hold::Mutate<Self::AccountId>
			+ fungible::freeze::Inspect<Self::AccountId>
			+ fungible::freeze::Mutate<Self::AccountId>;

		/// Type to access the Assets Pallet.
		type Fungibles: fungibles::Inspect<Self::AccountId, AssetId = u32, Balance = u128>
			+ fungibles::Mutate<Self::AccountId>
			+ fungibles::Create<Self::AccountId>;

		#[pallet::constant]
		type PalletId: Get<PalletId>;

		#[pallet::constant]
		type TokenDecimals: Get<u8>;
	}

	#[pallet::storage]
	pub type LiquidityPools<T: Config> =
		StorageMap<_, Blake2_128Concat, AssetPair<T>, LiquidityPool<T>>;

	#[pallet::storage]
	#[pallet::getter(fn asset_counter)]
	pub type AssetCounter<T: Config> = StorageValue<_, AssetIdOf<T>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event for a new liquidity pool creation
		LiquidityPoolCreated(AssetIdOf<T>, AssetIdOf<T>),
		/// Event for adding a liquidity to an existing pool
		LiquidityAdded(AssetIdOf<T>, AssetIdOf<T>, AssetBalanceOf<T>, AssetBalanceOf<T>),
		/// Event for removing a liquidity from an existing pool
		LiquidityRemoved(AssetIdOf<T>, AssetIdOf<T>, AssetBalanceOf<T>),
		/// Event for swapping exact in for min out
		SwappedExactIn(AssetIdOf<T>, AssetIdOf<T>, AssetBalanceOf<T>, AssetBalanceOf<T>),
		/// Event for swapping max of in for exact out
		SwappedExactOut(AssetIdOf<T>, AssetIdOf<T>, AssetBalanceOf<T>, AssetBalanceOf<T>),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// User balance insufficient to cover the cost of the operation
		InsufficientBalance,
		/// Invalid amount provided for the amount field
		InvalidAmount,
		/// Provided AssetId is not registered
		UnknownAssetId,
		/// Arithmetic overflow occurred during calculation
		StorageOverflow,
		/// Liquidity Pool does not exist
		LiquidityPoolDoesNotExist,
		/// Overflow for asset id counter
		AssetLimitReached,
		/// Arithmetic Error when multiplying and dividing
		Arithmetic,
		/// User provides insufficient amount_b that fails to maintain a constant token_a_reserve * token_b_reserve
		UnsufficientAmountB,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(Weight::default())]
		pub fn mint(
			origin: OriginFor<T>,
			asset_a: AssetIdOf<T>,
			asset_b: AssetIdOf<T>,
			amount_a: AssetBalanceOf<T>,
			amount_b: AssetBalanceOf<T>,
		) -> DispatchResult {
			// TODO: Make sure assets are sorted
			let who = ensure_signed(origin)?;

			let pool_asset_pair = AssetPair { asset_a: asset_a.clone(), asset_b: asset_b.clone() };

			let pallet_id: T::AccountId = T::PalletId::get().into_account_truncating();

			let pool = match LiquidityPools::<T>::get(pool_asset_pair.clone()) {
				Some(existing_pool) => existing_pool,
				None => {
					// Create the token for this pool
					let mut asset_counter = AssetCounter::<T>::get();

					// Create the asset with a specific asset_id
					T::Fungibles::create(
						asset_counter.clone(),
						pallet_id.clone(),
						true,
						AssetBalanceOf::<T>::one(),
					)?;

					// Create the liquidity pool if it doesn't exist
					let new_pool = LiquidityPool { id: asset_counter, manager: pallet_id };

					<LiquidityPools<T>>::set(pool_asset_pair.clone(), Some(new_pool.clone()));

					Self::deposit_event(crate::pallet::Event::LiquidityPoolCreated(
						asset_a, asset_b,
					));

					// Increment counter for keeping track of asset_id
					asset_counter =
						asset_counter.checked_add(1).ok_or(Error::<T>::AssetLimitReached)?;

					new_pool
				},
			};

			// Add initial liquidity
			pool.add_liquidity(pool_asset_pair, amount_a, amount_b, &who)?;

			Self::deposit_event(crate::pallet::Event::LiquidityAdded(
				asset_a, asset_b, amount_a, amount_b,
			));

			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(Weight::default())]
		pub fn burn(
			origin: OriginFor<T>,
			asset_a: AssetIdOf<T>,
			asset_b: AssetIdOf<T>,
			token_amount: AssetBalanceOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let pool_asset_pair = AssetPair { asset_a: asset_a.clone(), asset_b: asset_b.clone() };
			let pool = LiquidityPools::<T>::get(pool_asset_pair.clone())
				.ok_or_else(|| DispatchError::from(Error::<T>::LiquidityPoolDoesNotExist))?;
			pool.remove_liquidity(pool_asset_pair, token_amount, &who)?;

			// Self::deposit_event(Event::LiquidityRemoved());
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(Weight::default())]
		pub fn swap_exact_in_for_out(
			origin: OriginFor<T>,
			asset_in: AssetIdOf<T>,
			asset_out: AssetIdOf<T>,
			exact_in: AssetBalanceOf<T>,
			min_out: AssetBalanceOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(Weight::default())]
		pub fn swap_in_for_exact_out(
			origin: OriginFor<T>,
			asset_in: AssetIdOf<T>,
			asset_out: AssetIdOf<T>,
			max_in: AssetBalanceOf<T>,
			exact_out: AssetBalanceOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Ok(())
		}
	}
}
