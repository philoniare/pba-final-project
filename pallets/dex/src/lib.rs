#![cfg_attr(not(feature = "std"), no_std)]

use crate::types::LiquidityPool;
use frame_support::sp_runtime::traits::{One, Zero};
use frame_support::traits::fungibles;
use frame_support::PalletId;
pub use pallet::*;
use sp_runtime::Perbill;

mod types;
mod util;

#[cfg(test)]
pub(crate) mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
mod traits;

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
	use crate::types::AssetPair;
	use crate::*;
	use frame_support::{
		pallet_prelude::*,
		traits::{
			fungible::{self},
			fungibles::{self},
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

		#[pallet::constant]
		type MinimumLiquidity: Get<u32>;
	}

	#[pallet::storage]
	pub type LiquidityPools<T: Config> =
		StorageMap<_, Blake2_128Concat, AssetPair<T>, LiquidityPool<T>>;

	#[pallet::storage]
	#[pallet::getter(fn asset_counter)]
	pub type AssetCounter<T: Config> = StorageValue<_, AssetIdOf<T>>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event for a new liquidity pool creation
		LiquidityPoolCreated(AssetIdOf<T>, AssetIdOf<T>),
		/// Event for adding a liquidity to an existing pool
		LiquidityAdded(AssetIdOf<T>, AssetIdOf<T>, AssetBalanceOf<T>, AssetBalanceOf<T>),
		/// Event for removing a liquidity from an existing pool
		LiquidityRemoved(AssetIdOf<T>, AssetIdOf<T>, AssetBalanceOf<T>),
		/// Event for swapping first asset for second asset for the provided input amount
		Swapped(AssetIdOf<T>, AssetIdOf<T>, AssetBalanceOf<T>),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// There is not asset with the provided AssetId
		UnknownAssetId,
		/// Liquidity Pool does not exist
		LiquidityPoolDoesNotExist,
		/// Overflow for asset id counter
		AssetLimitReached,
		/// Arithmetic Error when multiplying and dividing
		Arithmetic,
		/// Liquidity Pool does not have sufficient liquidity for the specified swap
		InsufficientLiquidity,
		/// Insufficient Input Amount for a swap, please provide enough input amount
		InsufficientInputAmount,
		/// Attempted to burn a LP token with insufficient LP balance
		InsufficientBurnBalance,
		/// Provided assets are the same
		IdenticalAssets,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// The `mint` function allows a user to add liquidity to a liquidity pool.
		/// Given two assets and their amounts, it either creates a new liquidity pool if
		/// it does not already exist for these two assets or adds the provided liquidity
		/// to an existing pool. The user will receive LP tokens in return.
		///
		/// # Arguments
		///
		/// * `origin` - The origin caller of this function. This should be signed by the user adding liquidity.
		/// * `asset_a` - The identifier for the first type of asset that the user wants to provide.
		/// * `asset_b` - The identifier for the second type of asset that the user wants to provide.
		/// * `amount_a` - The amount of `asset_a` that the user is providing.
		/// * `amount_b` - The amount of `asset_b` that the user is providing.
		///
		/// # Errors
		///
		/// This function will return an error in the following scenarios:
		///
		/// * If the origin is not signed (i.e., the function was not called by a user).
		/// * If the provided assets do not exist.
		/// * If `asset_a` and `asset_b` are the same.
		/// * If `amount_a` or `amount_b` is 0 or less.
		/// * If creating a new liquidity pool would exceed the maximum number of allowed assets (`AssetLimitReached`).
		/// * If adding liquidity to the pool fails for any reason due to arithmetic overflows or underflows
		///
		/// # Events
		///
		/// If the function succeeds, it triggers two events:
		///
		/// * `LiquidityPoolCreated(asset_a, asset_b)` if a new liquidity pool was created.
		/// * `LiquidityAdded(asset_a, asset_b, amount_a, amount_b)` after the liquidity has been successfully added.
		///
		#[pallet::call_index(0)]
		#[pallet::weight(Weight::default())]
		pub fn mint(
			origin: OriginFor<T>,
			asset_a: AssetIdOf<T>,
			asset_b: AssetIdOf<T>,
			amount_a: AssetBalanceOf<T>,
			amount_b: AssetBalanceOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::ensure_assets_exist(asset_a, asset_b)?;
			// Assets should be different to create a pool
			ensure!(asset_a != asset_b, Error::<T>::IdenticalAssets);
			// Both amounts can be the same to create a liquidity pool
			ensure!(amount_a > AssetBalanceOf::<T>::zero(), Error::<T>::InsufficientInputAmount);
			ensure!(amount_b > AssetBalanceOf::<T>::zero(), Error::<T>::InsufficientInputAmount);

			let pool_asset_pair = AssetPair::new(asset_a.clone(), asset_b.clone());

			let pallet_id: T::AccountId = T::PalletId::get().into_account_truncating();

			let pool = match LiquidityPools::<T>::get(pool_asset_pair.clone()) {
				Some(existing_pool) => existing_pool,
				None => {
					// Create the token for this pool
					let mut asset_counter = match AssetCounter::<T>::get() {
						Some(current_count) => current_count,
						None => AssetIdOf::<T>::MAX,
					};

					// Create the asset with a specific asset_id
					T::Fungibles::create(
						asset_counter.clone(),
						pallet_id.clone(),
						true,
						AssetBalanceOf::<T>::one(),
					)?;

					// Create the liquidity pool if it doesn't exist
					let new_pool = LiquidityPool { id: asset_counter, manager: pallet_id };
					<LiquidityPools<T>>::set(&pool_asset_pair, Some(new_pool.clone()));

					Self::deposit_event(crate::pallet::Event::LiquidityPoolCreated(
						pool_asset_pair.asset_a,
						pool_asset_pair.asset_b,
					));

					// Increment counter for keeping track of asset_id
					asset_counter =
						asset_counter.checked_sub(1).ok_or(Error::<T>::AssetLimitReached)?;

					AssetCounter::<T>::set(Some(asset_counter));

					new_pool
				},
			};

			// Add liquidity
			pool.add_liquidity(&pool_asset_pair, amount_a, amount_b, &who)?;

			Self::deposit_event(Event::LiquidityAdded(
				pool_asset_pair.asset_a,
				pool_asset_pair.asset_b,
				amount_a,
				amount_b,
			));

			Ok(())
		}

		/// The `burn` function allows a user to remove liquidity from a specified
		/// liquidity pool. This is done by burning LP tokens and receiving the respective
		/// underlying assets in return.
		///
		/// # Arguments
		///
		/// * `origin` - The origin caller of this function. This should be signed by the user removing liquidity.
		/// * `asset_a` - The identifier for the first type of asset in the liquidity pool.
		/// * `asset_b` - The identifier for the second type of asset in the liquidity pool.
		/// * `token_amount` - The amount of liquidity the user wants to remove. This is denominated in LP tokens.
		///
		/// # Errors
		///
		/// This function will return an error in the following scenarios:
		///
		/// * If the origin is not signed (i.e., the function was not called by a user).
		/// * If the provided assets do not exist.
		/// * If `token_amount` is 0 or if it's more than the LP Token balance of the caller
		/// * If `asset_a` and `asset_b` are the same.
		/// * If the liquidity pool for the given asset pair does not exist.
		/// * If removing the liquidity from the pool fails for any reason due to arithmetic overflow or underflow
		///
		/// # Events
		///
		/// If the function succeeds, it triggers a `LiquidityRemoved(asset_a, asset_b, token_amount)` event.
		///
		#[pallet::call_index(1)]
		#[pallet::weight(Weight::default())]
		pub fn burn(
			origin: OriginFor<T>,
			asset_a: AssetIdOf<T>,
			asset_b: AssetIdOf<T>,
			token_amount: AssetBalanceOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::ensure_assets_exist(asset_a, asset_b)?;
			ensure!(asset_a != asset_b, Error::<T>::IdenticalAssets);
			// Make sure the pool exists
			let pool_asset_pair = AssetPair::new(asset_a.clone(), asset_b.clone());
			let pool = LiquidityPools::<T>::get(pool_asset_pair.clone())
				.ok_or_else(|| DispatchError::from(Error::<T>::LiquidityPoolDoesNotExist))?;

			pool.remove_liquidity(pool_asset_pair, token_amount, &who)?;

			Self::deposit_event(Event::LiquidityRemoved(asset_a, asset_b, token_amount));
			Ok(())
		}

		/// The `swap` function allows a user to exchange one type of token for another within a specific
		/// liquidity pool.
		///
		/// # Arguments
		///
		/// * `origin` - The origin caller of this function. This should be signed by the user performing the swap.
		/// * `asset_in` - The identifier for the type of asset that the user wants to swap from.
		/// * `asset_out` - The identifier for the type of asset that the user wants to swap to.
		/// * `amount_in` - The amount of `asset_in` that the user wants to swap.
		///
		/// # Errors
		///
		/// This function will return an error in the following scenarios:
		///
		/// * If the origin is not signed (i.e., the function was not called by a user).
		/// * If the provided assets do not exist.
		/// * If `amount_in` is 0 or less.
		/// * If `asset_in` and `asset_out` are the same.
		/// * If the liquidity pool for the given pair of assets does not exist.
		/// * If the swap operation fails for any reason due to arithmetic error
		///
		/// # Events
		///
		/// If the function succeeds, it triggers a `Swapped(asset_a, asset_b, amount_in)` event.
		///
		#[pallet::call_index(2)]
		#[pallet::weight(Weight::default())]
		pub fn swap(
			origin: OriginFor<T>,
			asset_in: AssetIdOf<T>,
			asset_out: AssetIdOf<T>,
			amount_in: AssetBalanceOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::ensure_assets_exist(asset_in, asset_out)?;
			ensure!(asset_in != asset_out, Error::<T>::IdenticalAssets);
			let pool_asset_pair = AssetPair::new(asset_in.clone(), asset_out.clone());
			let pool = LiquidityPools::<T>::get(pool_asset_pair.clone())
				.ok_or_else(|| DispatchError::from(Error::<T>::LiquidityPoolDoesNotExist))?;

			// Swapping for asset_in (asset_out) in the pool with amount_in of asset_in
			pool.swap(&who, pool_asset_pair.clone(), asset_in, asset_out, amount_in)?;

			Self::deposit_event(Event::Swapped(
				pool_asset_pair.asset_a,
				pool_asset_pair.asset_b,
				amount_in,
			));

			Ok(())
		}
	}

	impl<T: Config> traits::TokenRatio for Pallet<T> {
		type AssetId = AssetIdOf<T>;
		fn ratio(token_a: Self::AssetId, token_b: Self::AssetId) -> Result<Perbill, DispatchError> {
			let pool_key = AssetPair::new(token_a, token_b);
			Self::ensure_assets_exist(token_a, token_b)?;
			ensure!(token_a != token_b, Error::<T>::IdenticalAssets);
			let pool = <LiquidityPools<T>>::get(pool_key.clone())
				.ok_or_else(|| DispatchError::from(Error::<T>::LiquidityPoolDoesNotExist))?;

			let ratio_key = if token_a == pool_key.asset_a {
				pool_key
			} else {
				AssetPair { asset_a: token_a, asset_b: token_b }
			};
			let (token_a_reserve, token_b_reserve) = pool.get_reserve(&ratio_key)?;
			Self::calculate_perbill_ratio(token_a_reserve, token_b_reserve)
				.ok_or_else(|| DispatchError::from(Error::<T>::Arithmetic))
		}
	}

	impl<T: Config> traits::OraclePrice for Pallet<T> {
		type AssetId = AssetIdOf<T>;
		type Balance = AssetBalanceOf<T>;

		fn get_price_for(
			asset_in: Self::AssetId,
			amount_in: Self::Balance,
			asset_out: Self::AssetId,
		) -> Result<Self::Balance, DispatchError> {
			Self::ensure_assets_exist(asset_in, asset_out)?;
			ensure!(asset_in != asset_out, Error::<T>::IdenticalAssets);

			let pool_key = AssetPair::new(asset_in, asset_out);
			let pool = <LiquidityPools<T>>::get(pool_key.clone())
				.ok_or_else(|| DispatchError::from(Error::<T>::LiquidityPoolDoesNotExist))?;

			let (reserve_in, reserve_out) =
				pool.get_reserve(&AssetPair { asset_a: asset_in, asset_b: asset_out })?;
			pool.calculate_output_for(amount_in, reserve_in, reserve_out)
		}
	}
}
