use crate::*;
use sp_runtime::{DispatchError, Perbill};

impl<T: Config> Pallet<T> {
	pub(super) fn expand_to_decimals(n: AssetBalanceOf<T>) -> AssetBalanceOf<T> {
		n * 10u128.pow(10u32)
	}

	pub(super) fn calculate_perbill_ratio(numerator: u128, denominator: u128) -> Option<Perbill> {
		if denominator == 0 {
			return None;
		}

		let ratio = numerator as u128 * 1_000_000_000u128 / denominator as u128;

		Some(Perbill::from_parts(ratio.min(u32::MAX as u128) as u32))
	}
}

impl<T: Config> LiquidityPool<T> {
	fn checked_operation<F, R>(x: R, y: R, func: F) -> Result<R, DispatchError>
	where
		F: Fn(R, R) -> Option<R>,
		R: sp_runtime::traits::AtLeast32BitUnsigned,
	{
		func(x, y).ok_or_else(|| Error::<T>::Arithmetic.into())
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
