use crate::*;
use frame_support::ensure;
use frame_support::traits::tokens::{Fortitude, Precision, Preservation};
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use rust_decimal::Decimal;
use sp_runtime::{DispatchError, Perbill};

impl<T: Config> Pallet<T> {
	pub(super) fn expand_to_decimals(n: AssetBalanceOf<T>) -> AssetBalanceOf<T> {
		n * 10u128.pow(10u32)
	}

	pub(super) fn decimals_to_numeric(n: AssetBalanceOf<T>) -> AssetBalanceOf<T> {
		let decimal = Decimal::from_u128(n).expect("already a u128; qed");
		let numerator: Decimal = 10u128.pow(10u32).into();
		(decimal / numerator)
			.round()
			.to_u128()
			.expect("only called from tests; can panic")
	}

	pub(super) fn calculate_perbill_ratio(numerator: u128, denominator: u128) -> Option<Perbill> {
		if denominator == 0 {
			return None;
		}

		let ratio = numerator as u128 * 1_000_000_000u128 / denominator as u128;

		Some(Perbill::from_parts(ratio.min(u32::MAX as u128) as u32))
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
	pub(super) fn transfer_in(
		&self,
		asset: AssetIdOf<T>,
		from: &AccountIdOf<T>,
		amount: AssetBalanceOf<T>,
	) -> Result<u128, DispatchError> {
		T::Fungibles::transfer(asset, from, &self.manager, amount, Preservation::Expendable)
	}

	pub(super) fn transfer_out(
		&self,
		asset: AssetIdOf<T>,
		to: &AccountIdOf<T>,
		amount: AssetBalanceOf<T>,
	) -> Result<u128, DispatchError> {
		T::Fungibles::transfer(asset, &self.manager, to, amount, Preservation::Expendable)
	}

	pub(super) fn burn_lp(
		&self,
		who: &AccountIdOf<T>,
		amount: AssetBalanceOf<T>,
	) -> Result<u128, DispatchError> {
		let lp_balance = T::Fungibles::balance(self.id, &who);
		ensure!(lp_balance >= amount, Error::<T>::InsufficientBurnBalance);
		T::Fungibles::burn_from(self.id, who, amount, Precision::Exact, Fortitude::Polite)
	}

	fn checked_operation<F, R>(x: R, y: R, func: F) -> Result<R, DispatchError>
	where
		F: Fn(R, R) -> Option<R>,
		R: sp_runtime::traits::AtLeast32BitUnsigned,
	{
		func(x, y).ok_or(Error::<T>::Arithmetic.into())
	}

	pub(super) fn safe_mul(
		x: AssetBalanceOf<T>,
		y: AssetBalanceOf<T>,
	) -> Result<AssetBalanceOf<T>, DispatchError> {
		Self::checked_operation(x, y, AssetBalanceOf::<T>::checked_mul)
	}

	pub(super) fn safe_div(
		x: AssetBalanceOf<T>,
		y: AssetBalanceOf<T>,
	) -> Result<AssetBalanceOf<T>, DispatchError> {
		Self::checked_operation(x, y, AssetBalanceOf::<T>::checked_div)
	}

	pub(super) fn safe_add(
		x: AssetBalanceOf<T>,
		y: AssetBalanceOf<T>,
	) -> Result<AssetBalanceOf<T>, DispatchError> {
		Self::checked_operation(x, y, AssetBalanceOf::<T>::checked_add)
	}

	pub(super) fn safe_sub(
		x: AssetBalanceOf<T>,
		y: AssetBalanceOf<T>,
	) -> Result<AssetBalanceOf<T>, DispatchError> {
		Self::checked_operation(x, y, AssetBalanceOf::<T>::checked_sub)
	}
}
