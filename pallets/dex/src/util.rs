use crate::*;
use frame_support::ensure;
use frame_support::traits::tokens::{Fortitude, Precision, Preservation};
use sp_runtime::traits::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub};
use sp_runtime::{DispatchError, FixedPointNumber, FixedU128, Perbill};

impl<T: Config> Pallet<T> {
	pub(super) fn calculate_perbill_ratio(
		numerator: AssetBalanceOf<T>,
		denominator: AssetBalanceOf<T>,
	) -> Result<Perbill, DispatchError> {
		if denominator == AssetBalanceOf::<T>::zero() {
			return Err(DispatchError::from(Error::<T>::Arithmetic));
		}

		let decimals = T::TokenDecimals::get();
		let multiplier: AssetBalanceOf<T> = 10u32.pow(decimals).into();
		let product = numerator
			.checked_mul(&multiplier)
			.ok_or_else(|| DispatchError::from(Error::<T>::Arithmetic))?;
		let ratio = product
			.checked_div(&denominator)
			.ok_or_else(|| DispatchError::from(Error::<T>::Arithmetic))?;

		// Ok(Perbill::from_parts(ratio))
		Ok(Perbill::from_parts(0))
	}

	pub(super) fn ensure_assets_exist(
		asset_a: AssetIdOf<T>,
		asset_b: AssetIdOf<T>,
	) -> Result<(), DispatchError> {
		ensure!(T::Fungibles::asset_exists(asset_a), Error::<T>::UnknownAssetId);
		ensure!(T::Fungibles::asset_exists(asset_b), Error::<T>::UnknownAssetId);
		Ok(())
	}
}

impl<T: Config> LiquidityPool<T> {
	fn checked_operation<F, R>(x: &R, y: &R, func: F) -> Result<R, DispatchError>
	where
		F: Fn(&R, &R) -> Option<R>,
		R: sp_runtime::traits::AtLeast32BitUnsigned,
	{
		func(x, y).ok_or(Error::<T>::Arithmetic.into())
	}

	pub(super) fn safe_mul(
		x: AssetBalanceOf<T>,
		y: AssetBalanceOf<T>,
	) -> Result<AssetBalanceOf<T>, DispatchError> {
		Self::checked_operation(&x, &y, |a, b| AssetBalanceOf::<T>::checked_mul(a, b))
	}

	pub(super) fn safe_div(
		x: AssetBalanceOf<T>,
		y: AssetBalanceOf<T>,
	) -> Result<AssetBalanceOf<T>, DispatchError> {
		Self::checked_operation(&x, &y, |a, b| AssetBalanceOf::<T>::checked_div(a, b))
	}

	pub(super) fn safe_add(
		x: AssetBalanceOf<T>,
		y: AssetBalanceOf<T>,
	) -> Result<AssetBalanceOf<T>, DispatchError> {
		Self::checked_operation(&x, &y, |a, b| AssetBalanceOf::<T>::checked_add(a, b))
	}

	pub(super) fn safe_sub(
		x: AssetBalanceOf<T>,
		y: AssetBalanceOf<T>,
	) -> Result<AssetBalanceOf<T>, DispatchError> {
		Self::checked_operation(&x, &y, |a, b| AssetBalanceOf::<T>::checked_sub(a, b))
	}

	pub(super) fn transfer_in(
		&self,
		asset: AssetIdOf<T>,
		from: &AccountIdOf<T>,
		amount: AssetBalanceOf<T>,
	) -> Result<AssetBalanceOf<T>, DispatchError> {
		T::Fungibles::transfer(asset, from, &self.manager, amount, Preservation::Expendable)
	}

	pub(super) fn transfer_out(
		&self,
		asset: AssetIdOf<T>,
		to: &AccountIdOf<T>,
		amount: AssetBalanceOf<T>,
	) -> Result<AssetBalanceOf<T>, DispatchError> {
		T::Fungibles::transfer(asset, &self.manager, to, amount, Preservation::Expendable)
	}

	pub(super) fn burn_lp(
		&self,
		who: &AccountIdOf<T>,
		amount: AssetBalanceOf<T>,
	) -> Result<AssetBalanceOf<T>, DispatchError> {
		let lp_balance = T::Fungibles::balance(self.id, &who);
		ensure!(lp_balance >= amount, Error::<T>::InsufficientBurnBalance);
		T::Fungibles::burn_from(self.id, who, amount, Precision::Exact, Fortitude::Polite)
	}
}
