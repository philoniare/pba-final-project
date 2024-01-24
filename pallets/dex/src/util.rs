use crate::*;
use sp_runtime::Perbill;

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
