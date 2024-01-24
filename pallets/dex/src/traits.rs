use sp_runtime::DispatchError;
pub use sp_runtime::Perbill;

pub trait TokenRatio {
	type AssetId;
	fn ratio(token_a: Self::AssetId, token_b: Self::AssetId) -> Result<Perbill, DispatchError>;
}

pub trait OraclePrice {
	type AssetId;
	type Balance;

	fn get_price_for(
		asset_in: Self::AssetId,
		amount_in: Self::Balance,
		asset_out: Self::AssetId,
	) -> Result<Self::Balance, DispatchError>;
}
