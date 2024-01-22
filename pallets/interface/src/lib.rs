#![cfg_attr(not(feature = "std"), no_std)]

// Note that these interfaces should not limit or heavily influence the design of your pallet.
//
// These interfaces do NOT make sense to expose as the extrinsics of your pallet.
// Instead, these will simply be used to execute unit tests to verify the basic logic of your
// pallet is working. You should design your own extrinsic functions which make sense for
// exposing to end users of your pallet.
//
// It should be totally possible to create more complex or unique pallets, while adhering to
// the interfaces below.
//
// If any of these interfaces are not compatible with your design or vision, talk to an
// instructor and we can figure out the best way forward.

use core::{cmp::Ord, fmt::Debug};
use frame_support::{
	dispatch::Vec,
	pallet_prelude::{
		DispatchError, DispatchResult, MaxEncodedLen, MaybeSerializeDeserialize, Member, Parameter,
	},
	sp_runtime::Perbill,
	traits::tokens::{AssetId as AssetIdTrait, Balance as BalanceTrait},
};

/// A minimal interface to test the functionality of the Voting pallet.
pub trait VotingInterface {
	/// The type which can be used to identify accounts.
	type AccountId: Parameter + Member + MaybeSerializeDeserialize + Debug + Ord + MaxEncodedLen;
	/// The type representing the balance users can vote with.
	type VotingBalance: BalanceTrait;
	/// The type representing a unique ID for a proposal.
	type ProposalId: Parameter + Member + MaybeSerializeDeserialize + Debug + Ord + MaxEncodedLen;

	/// This function should register a user in the identity system, allowing that user to vote, and
	/// give that user some voting balance equal to `amount`.
	fn add_voter(who: Self::AccountId, amount: Self::VotingBalance) -> DispatchResult;

	/// Create a proposal with the following metadata.
	///
	/// If `Ok`, return the `ProposalId`.
	fn create_proposal(metadata: Vec<u8>) -> Result<Self::ProposalId, DispatchError>;

	/// Make a voter vote on a proposal with a given vote weight.
	///
	/// If the voter supports the proposal, they will vote `aye = true`, otherwise they should vote
	/// `aye = false`.
	///
	/// The `vote_weight` should represent the value after we take the sqrt of their voting balance,
	/// thus you can simply square the `amount` rather than taking the sqrt of some value.
	///
	/// For example: If a user votes with `vote_weight = 10`, then we should check they have at
	/// least `100` total voting balance.
	fn vote(
		proposal: Self::ProposalId,
		voter: Self::AccountId,
		aye: bool,
		vote_weight: Self::VotingBalance,
	) -> DispatchResult;

	/// Do whatever is needed to resolve the vote, and determine the outcome.
	///
	/// If `Ok`, return the result of the vote with a bool, `true` being the vote passed, and
	/// `false` being the vote failed.
	fn close_vote(proposal: Self::ProposalId) -> Result<bool, DispatchError>;
}

/// A minimal interface to test the functionality of the DPOS pallet.
pub trait DposInterface {
	/// The type which can be used to identify accounts.
	type AccountId: Parameter + Member + MaybeSerializeDeserialize + Debug + Ord + MaxEncodedLen;
	/// The underlying balance type of the NativeBalance.
	type StakingBalance: BalanceTrait;

	/// A helper function which should give a new user a balance they can use in the staking system.
	fn setup_account(who: Self::AccountId, amount: Self::StakingBalance) -> DispatchResult;

	/// Get the balance of any account.
	fn balance(who: Self::AccountId) -> Self::StakingBalance;

	/// Register a user to be a validator.
	fn register_validator(who: Self::AccountId) -> DispatchResult;

	/// Any user with a balance can delegate. They choose who they wan to delegate to, and how much
	/// they want to stake.
	fn delegate(
		delegator: Self::AccountId,
		validator: Self::AccountId,
		amount: Self::StakingBalance,
	) -> DispatchResult;

	/// Get a new list of the winning validators with the highest stake to use in the next staking
	/// session.
	///
	/// Should return up to `max_validators` in the vector.
	fn get_winning_validators(max_validators: u32) -> Result<Vec<Self::AccountId>, DispatchError>;

	/// Query the total amount of stake backing a validator.
	fn get_validator_stake(who: Self::AccountId) -> Option<Self::StakingBalance>;
}

/// A minimal interface to test the functionality of the DEX Pallet.
pub trait DexInterface {
	/// The type which can be used to identify accounts.
	type AccountId: Parameter + Member + MaybeSerializeDeserialize + Debug + Ord + MaxEncodedLen;
	/// The type used to identify various fungible assets.
	type AssetId: AssetIdTrait;
	/// The type used to represent the balance of a fungible asset.
	type AssetBalance: BalanceTrait;

	/// A helper function to setup an account so it can hold any number of assets.
	fn setup_account(who: Self::AccountId) -> DispatchResult;

	/// Do whatever is needed to give user some amount of an asset.
	fn mint_asset(
		who: Self::AccountId,
		token_id: Self::AssetId,
		amount: Self::AssetBalance,
	) -> DispatchResult;

	/// Get a user's asset balance.
	fn asset_balance(who: Self::AccountId, token_id: Self::AssetId) -> Self::AssetBalance;

	/// Return the swap fee as a percentage.
	fn swap_fee() -> Perbill;

	/// Get the LP Token ID that will be generated by creating a pool of `asset_a` and `asset_b`.
	fn lp_id(asset_a: Self::AssetId, asset_b: Self::AssetId) -> Self::AssetId;

	/// Create a liquidity pool, initiated by `who`.
	fn create_liquidity_pool(
		who: Self::AccountId,
		asset_a: Self::AssetId,
		asset_b: Self::AssetId,
		amount_a: Self::AssetBalance,
		amount_b: Self::AssetBalance,
	) -> DispatchResult;

	/// Add liquidity to a pool on behalf of the user. If needed this will create the pool.
	///
	/// LP tokens are minted to the caller which are used to represent
	/// "ownership" of the pool.
	fn add_liquidity(
		who: Self::AccountId,
		asset_a: Self::AssetId,
		asset_b: Self::AssetId,
		amount_a: Self::AssetBalance,
		amount_b: Self::AssetBalance,
	) -> DispatchResult;

	/// Removes liquidity from the pool on behalf of the user.
	///
	/// `token_amount` represents the amount of LP tokens to be burned in exchange for underlying
	/// assets.
	fn remove_liquidity(
		who: Self::AccountId,
		asset_a: Self::AssetId,
		asset_b: Self::AssetId,
		token_amount: Self::AssetBalance,
	) -> DispatchResult;

	/// Swaps an exact amount of `asset_in` for a minimum amount of `asset_out` on behalf of `who`.
	///
	/// The swap fee is deducted from the out amount, so it is left in
	/// the pool for LPs.
	fn swap_exact_in_for_out(
		who: Self::AccountId,
		asset_in: Self::AssetId,
		asset_out: Self::AssetId,
		exact_in: Self::AssetBalance,
		min_out: Self::AssetBalance,
	) -> DispatchResult;

	/// Swaps a max amount of `asset_in` for an exact amount of `asset_out` on behalf of `who`.
	///
	/// The swap fee is added to the in amount, and left in the pool for
	/// the LPs.
	fn swap_in_for_exact_out(
		origin: Self::AccountId,
		asset_in: Self::AssetId,
		asset_out: Self::AssetId,
		max_in: Self::AssetBalance,
		exact_out: Self::AssetBalance,
	) -> DispatchResult;
}
