use crate::*;

impl<T: Config> Pallet<T> {
	pub fn expand_to_18_decimals(n: AssetBalanceOf<T>) -> AssetBalanceOf<T> {
		n * 10u128.pow(5u32)
	}
}
