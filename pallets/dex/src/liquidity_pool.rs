use super::*;
use frame_support::pallet_prelude::*;
use frame_support::traits::tokens::{Fortitude, Precision, Preservation};
use sp_runtime::helpers_128bit::sqrt;
use sp_runtime::traits::{CheckedDiv, CheckedMul};
use sp_runtime::Perbill;
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
	pub fn remove_liquidity(
		&self,
		asset_pair: AssetPair<T>,
		liquidity: AssetBalanceOf<T>,
		who: &AccountIdOf<T>,
	) -> DispatchResult {
		let total_issuance = T::Fungibles::total_issuance(self.id);
		let token_a_reserve = T::Fungibles::balance(asset_pair.asset_a, &self.manager);
		let token_b_reserve = T::Fungibles::balance(asset_pair.asset_b, &self.manager);

		let ratio_a = Self::safe_mul(liquidity, token_a_reserve)?;
		let ratio_b = Self::safe_mul(liquidity, token_b_reserve)?;
		let amount_a = Self::safe_div(ratio_a, total_issuance)?;
		let amount_b = Self::safe_div(ratio_b, total_issuance)?;

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

	pub fn calculate_output_for(
		&self,
		amount_in: AssetBalanceOf<T>,
		reserve_in: AssetBalanceOf<T>,
		reserve_out: AssetBalanceOf<T>,
	) -> Result<AssetBalanceOf<T>, DispatchError> {
		if reserve_in.is_zero() || reserve_out.is_zero() {
			return Ok(AssetBalanceOf::<T>::zero());
		}

		let amount_without_fee = Self::safe_mul(amount_in, 997u128)?;
		let ratio = Self::safe_mul(amount_without_fee, reserve_out)?;
		let mut reserve_total = Self::safe_mul(reserve_in, 1000u128)?;
		reserve_total = Self::safe_add(reserve_total, amount_without_fee)?;
		let total = Self::safe_div(ratio, reserve_total)?;

		Ok(total)
	}

	fn calculate_liquidity(
		&self,
		total_issuance: AssetBalanceOf<T>,
		amount_a: AssetBalanceOf<T>,
		amount_b: AssetBalanceOf<T>,
		token_a_reserve: AssetBalanceOf<T>,
		token_b_reserve: AssetBalanceOf<T>,
	) -> Result<AssetBalanceOf<T>, DispatchError> {
		let zero_balance = AssetBalanceOf::<T>::zero();
		let mut liquidity = zero_balance;

		if total_issuance == zero_balance {
			let product = Self::safe_mul(amount_a, amount_b)?;
			liquidity = Self::safe_sub(sqrt(product), u128::from(T::MinimumLiquidity::get()))?;
			T::Fungibles::mint_into(
				self.id,
				&self.manager,
				u128::from(T::MinimumLiquidity::get()),
			)?;
		} else {
			// Get current reserved amounts for each asset
			let a_ratio = Self::safe_mul(amount_a, total_issuance)?;
			let token_a_amount = Self::safe_div(a_ratio, token_a_reserve)?;

			let b_ratio = Self::safe_mul(amount_b, total_issuance)?;
			let token_b_amount = Self::safe_div(b_ratio, token_b_reserve)?;

			liquidity = min(token_a_amount, token_b_amount);
		}

		Ok(liquidity)
	}

	pub fn add_liquidity(
		&self,
		asset_pair: AssetPair<T>,
		amount_a: AssetBalanceOf<T>,
		amount_b: AssetBalanceOf<T>,
		who: &AccountIdOf<T>,
	) -> DispatchResult {
		ensure!(amount_a > AssetBalanceOf::<T>::zero(), Error::<T>::InsufficientInputAmount);
		ensure!(amount_b > AssetBalanceOf::<T>::zero(), Error::<T>::InsufficientInputAmount);

		let total_issuance = T::Fungibles::total_issuance(self.id);
		let token_a_reserve = T::Fungibles::balance(asset_pair.asset_a, &self.manager);
		let token_b_reserve = T::Fungibles::balance(asset_pair.asset_b, &self.manager);

		let liquidity = self.calculate_liquidity(
			total_issuance,
			amount_a,
			amount_b,
			token_a_reserve,
			token_b_reserve,
		)?;
		ensure!(liquidity > AssetBalanceOf::<T>::zero(), Error::<T>::UnsufficientAmountB);

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
