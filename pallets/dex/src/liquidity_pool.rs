use super::*;
use frame_support::pallet_prelude::*;
use frame_support::traits::tokens::{Fortitude, Precision, Preservation};
use sp_runtime::helpers_128bit::sqrt;
use sp_runtime::traits::{CheckedDiv, CheckedMul};
use std::cmp::min;

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct AssetPair<T: Config> {
	pub asset_a: AssetIdOf<T>,
	pub asset_b: AssetIdOf<T>,
}

impl<T: Config> AssetPair<T> {
	pub fn new(asset_one: AssetIdOf<T>, asset_two: AssetIdOf<T>) -> Self {
		if asset_one <= asset_two {
			AssetPair { asset_a: asset_one, asset_b: asset_two }
		} else {
			AssetPair { asset_a: asset_two, asset_b: asset_one }
		}
	}
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct LiquidityPool<T: Config> {
	pub id: AssetIdOf<T>,
	pub manager: AccountIdOf<T>,
}

impl<T: Config> LiquidityPool<T> {
	fn calculate_liquidity(
		&self,
		amount_a: AssetBalanceOf<T>,
		amount_b: AssetBalanceOf<T>,
	) -> Result<AssetBalanceOf<T>, sp_runtime::DispatchError> {
		let min_liquidity = u128::from(T::MinimumLiquidity::get());
		let product = amount_a
			.checked_mul(amount_b)
			.ok_or_else(|| DispatchError::from(Error::<T>::Arithmetic))?;
		// Initial minimum liquidity is locked away
		let liquidity = sqrt(product)
			.checked_sub(min_liquidity)
			.ok_or_else(|| DispatchError::from(Error::<T>::Arithmetic))?;
		T::Fungibles::mint_into(self.id, &self.manager, min_liquidity)?;

		Ok(liquidity)
	}

	pub fn remove_liquidity(
		&self,
		asset_pair: AssetPair<T>,
		liquidity: AssetBalanceOf<T>,
		who: &AccountIdOf<T>,
	) -> DispatchResult {
		let total_issuance = T::Fungibles::total_issuance(self.id);
		let token_a_reserve = T::Fungibles::balance(asset_pair.asset_a, &self.manager);
		let token_b_reserve = T::Fungibles::balance(asset_pair.asset_b, &self.manager);

		let ratio_a = liquidity
			.checked_mul(token_a_reserve)
			.ok_or_else(|| DispatchError::from(Error::<T>::Arithmetic))?;
		let ratio_b = liquidity
			.checked_mul(token_b_reserve)
			.ok_or_else(|| DispatchError::from(Error::<T>::Arithmetic))?;
		let amount_a = ratio_a
			.checked_div(total_issuance)
			.ok_or_else(|| DispatchError::from(Error::<T>::Arithmetic))?;
		let amount_b = ratio_b
			.checked_div(total_issuance)
			.ok_or_else(|| DispatchError::from(Error::<T>::Arithmetic))?;

		// Burn the LP token
		T::Fungibles::burn_from(self.id, who, liquidity, Precision::Exact, Fortitude::Polite)?;

		// Transfer back assets to the liquidity provider
		T::Fungibles::transfer(
			asset_pair.asset_a,
			&self.manager,
			who,
			amount_a,
			Preservation::Expendable,
		)?;
		T::Fungibles::transfer(
			asset_pair.asset_b,
			&self.manager,
			who,
			amount_b,
			Preservation::Expendable,
		)?;

		Ok(())
	}

	pub fn swap(
		&self,
		who: &AccountIdOf<T>,
		asset_pair: AssetPair<T>,
		amount_a_out: AssetBalanceOf<T>,
		amount_b_out: AssetBalanceOf<T>,
	) -> DispatchResult {
		Ok(())
	}

	pub fn calc_output(
		&self,
		amount_in: AssetBalanceOf<T>,
		reserve_in: AssetBalanceOf<T>,
		reserve_out: AssetBalanceOf<T>,
	) -> Result<AssetBalanceOf<T>, sp_runtime::DispatchError> {
		if reserve_in == 0 || reserve_out == 0 {
			return Ok(AssetBalanceOf::<T>::zero());
		}

		let amount_without_fee =
			amount_in.checked_mul(997u128).ok_or_else(|| Error::<T>::Arithmetic)?;
		let ratio = amount_without_fee
			.checked_mul(reserve_out)
			.ok_or_else(|| Error::<T>::Arithmetic)?;
		let mut reserve_total =
			reserve_in.checked_mul(1000u128).ok_or_else(|| Error::<T>::Arithmetic)?;
		reserve_total = reserve_total
			.checked_add(amount_without_fee)
			.ok_or_else(|| Error::<T>::Arithmetic)?;
		let total = ratio.checked_div(reserve_total).ok_or_else(|| Error::<T>::Arithmetic)?;
		Ok(total)
	}

	pub fn add_liquidity(
		&self,
		asset_pair: AssetPair<T>,
		amount_a: AssetBalanceOf<T>,
		amount_b: AssetBalanceOf<T>,
		who: &AccountIdOf<T>,
	) -> DispatchResult {
		let total_issuance = T::Fungibles::total_issuance(self.id);
		let token_a_reserve = T::Fungibles::balance(asset_pair.asset_a, &self.manager);
		let token_b_reserve = T::Fungibles::balance(asset_pair.asset_b, &self.manager);
		let mut liquidity = 0u128;
		if total_issuance == <AssetBalanceOf<T>>::default() {
			liquidity = self.calculate_liquidity(amount_a, amount_b)?;
		} else {
			// Get current reserved amounts for each asset
			let a_ratio =
				amount_a.checked_mul(total_issuance).ok_or_else(|| Error::<T>::Arithmetic)?;

			let token_a_amount =
				a_ratio.checked_div(token_a_reserve).ok_or_else(|| Error::<T>::Arithmetic)?;

			let b_ratio =
				amount_b.checked_mul(total_issuance).ok_or_else(|| Error::<T>::Arithmetic)?;

			let token_b_amount =
				b_ratio.checked_div(token_b_reserve).ok_or_else(|| Error::<T>::Arithmetic)?;

			liquidity = min(token_a_amount, token_b_amount);
		}
		ensure!(liquidity > 0, Error::<T>::UnsufficientAmountB);

		T::Fungibles::mint_into(self.id, who, liquidity)?;
		T::Fungibles::transfer(
			asset_pair.asset_a,
			&who,
			&self.manager,
			amount_a,
			Preservation::Expendable,
		)?;
		T::Fungibles::transfer(
			asset_pair.asset_b,
			&who,
			&self.manager,
			amount_b,
			Preservation::Expendable,
		)?;

		Ok(())
	}
}
