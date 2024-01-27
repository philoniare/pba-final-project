use super::*;
use sp_runtime::traits::IntegerSquareRoot;
use sp_std::cmp::min;

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
			// Swap the two for ordering
			AssetPair { asset_a: asset_two, asset_b: asset_one }
		}
	}
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct LiquidityPool<T: Config> {
	pub id: AssetIdOf<T>,
	pub manager: AccountIdOf<T>,
	pub asset_a_balance: AssetBalanceOf<T>,
	pub asset_b_balance: AssetBalanceOf<T>,
}

impl<T: Config> LiquidityPool<T> {
	pub fn remove_liquidity(
		&mut self,
		asset_pair: &AssetPair<T>,
		liquidity: AssetBalanceOf<T>,
		who: &AccountIdOf<T>,
	) -> DispatchResult {
		// Calculate the amount of assets to return to the liquidity provider
		let total_issuance = T::Fungibles::total_issuance(self.id.clone());
		let ratio_a = Self::safe_mul(liquidity, self.asset_a_balance)?;
		let ratio_b = Self::safe_mul(liquidity, self.asset_b_balance)?;
		let amount_a = Self::safe_div(ratio_a, total_issuance)?;
		let amount_b = Self::safe_div(ratio_b, total_issuance)?;

		// Burn the LP token
		self.burn_lp(&who, liquidity)?;

		// Transfer back assets to the liquidity provider
		self.transfer_out(asset_pair.asset_a, &who, amount_a)?;
		self.transfer_out(asset_pair.asset_b, &who, amount_b)?;

		// Update internal balances of the pool
		self.asset_a_balance = Self::safe_sub(self.asset_a_balance, amount_a)?;
		self.asset_b_balance = Self::safe_sub(self.asset_b_balance, amount_b)?;

		Ok(())
	}

	pub fn swap(
		&self,
		who: &AccountIdOf<T>,
		asset_pair: AssetPair<T>,
		asset_in: AssetIdOf<T>,
		asset_out: AssetIdOf<T>,
		amount_in: AssetBalanceOf<T>,
	) -> DispatchResult {
		let (token_in_reserve, token_out_reserve) = if asset_out == asset_pair.asset_a {
			// Rotate the assets in case they want to reverse swap
			(self.asset_b_balance, self.asset_a_balance)
		} else {
			(self.asset_a_balance, self.asset_b_balance)
		};
		ensure!(amount_in > AssetBalanceOf::<T>::zero(), Error::<T>::InsufficientInputAmount);
		ensure!(
			token_in_reserve > amount_in && token_out_reserve > AssetBalanceOf::<T>::zero(),
			Error::<T>::InsufficientLiquidity
		);

		// Swap the assets
		let amount_out =
			self.calculate_output_for(amount_in, token_in_reserve - amount_in, token_out_reserve)?;
		self.transfer_in(asset_in, &who, amount_in)?;
		self.transfer_out(asset_out, &who, amount_out)?;

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

		// Deduct fixed 0.3% fee from the swap, which is used to reward liquidity providers
		let amount_without_fee = Self::safe_mul(amount_in, 997u32.into())?;
		let ratio = Self::safe_mul(amount_without_fee, reserve_out)?;
		let mut reserve_total = Self::safe_mul(reserve_in, 1000u32.into())?;
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

		return if total_issuance == zero_balance {
			// If there is no liquidity minted yet, then the liquidity is the
			// sqrt of the product of the two amounts
			let product = Self::safe_mul(amount_a, amount_b)?;
			let min_liq: AssetBalanceOf<T> = T::MinimumLiquidity::get().into();
			let product_sqrt = product.integer_sqrt();
			ensure!(product_sqrt >= min_liq.into(), Error::<T>::InsufficientLiquidity);
			let liquidity = Self::safe_sub(product_sqrt, min_liq)?;
			T::Fungibles::mint_into(
				self.id.clone(),
				&self.manager,
				T::MinimumLiquidity::get().into(),
			)?;
			Ok(liquidity)
		} else {
			// When there is liquidity, calculate token_amount
			// based on the reserved amounts for each asset
			let a_ratio = Self::safe_mul(amount_a, total_issuance)?;
			let token_a_amount = Self::safe_div(a_ratio, token_a_reserve)?;

			let b_ratio = Self::safe_mul(amount_b, total_issuance)?;
			let token_b_amount = Self::safe_div(b_ratio, token_b_reserve)?;

			Ok(min(token_a_amount, token_b_amount))
		};
	}

	pub fn add_liquidity(
		&mut self,
		asset_pair: &AssetPair<T>,
		amount_a: AssetBalanceOf<T>,
		amount_b: AssetBalanceOf<T>,
		who: &AccountIdOf<T>,
	) -> DispatchResult {
		let total_issuance = T::Fungibles::total_issuance(self.id.clone());

		let liquidity = self.calculate_liquidity(
			total_issuance,
			amount_a,
			amount_b,
			self.asset_a_balance,
			self.asset_b_balance,
		)?;
		ensure!(liquidity > AssetBalanceOf::<T>::zero(), Error::<T>::InsufficientLiquidity);

		// Mint & Send LP Token to the caller
		T::Fungibles::mint_into(self.id.clone(), who, liquidity)?;
		// Transfer provided tokens to the pool
		self.transfer_in(asset_pair.asset_a.clone(), &who, amount_a)?;
		self.transfer_in(asset_pair.asset_b.clone(), &who, amount_b)?;
		// Update the balances in the pool
		self.asset_a_balance = Self::safe_add(self.asset_a_balance, amount_a)?;
		self.asset_b_balance = Self::safe_add(self.asset_b_balance, amount_b)?;

		Ok(())
	}
}
