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
		let product = amount_a
			.checked_mul(amount_b)
			.ok_or_else(|| DispatchError::from(Error::<T>::Arithmetic))?;
		let liquidity = sqrt(product);

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
