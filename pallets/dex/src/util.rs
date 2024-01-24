use crate::*;

impl<T: Config> Pallet<T> {
	pub(super) fn expand_to_decimals(n: AssetBalanceOf<T>) -> AssetBalanceOf<T> {
		n * 10u128.pow(10u32)
	}
}
