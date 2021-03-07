use codec::FullCodec;
pub use frame_support::traits::{BalanceStatus, LockIdentifier};
use sp_runtime::{
	traits::{
		AtLeast32BitUnsigned, MaybeSerializeDeserialize
	}, 
	DispatchResult
};
use sp_std::{
	cmp::{Eq, PartialEq},
	fmt::Debug,
};

	/// The frequency of adjustments for the Currency supply.
pub struct ElastAdjustmentFrequency<BlockNumber> {
	/// Number of blocks for adjustment frequency.
	pub adjustment_frequency: BlockNumber,
}

/// Abstraction over a fungible multi-stable-currency Token Elasticity of Supply system.

pub trait SerpTes<AccountId, BlockNumber, CurrencyId, Price> {
	/// The currency identifier.
	type CurrencyId: FullCodec + Eq + PartialEq + Copy + MaybeSerializeDeserialize + Debug;

	/// The balance of an account.
	type Balance: AtLeast32BitUnsigned + FullCodec + Copy + MaybeSerializeDeserialize + Debug + Default;
    
	// Public immutables

	/// The total amount of issuance of `currency_id`.
	fn total_issuance(currency_id: Self::CurrencyId) -> Self::Balance;

	// Public mutables

	/// Contract the currency supply.
	fn contract_supply(
		currency_id: Self::CurrencyId,
		total_issuance: Self::Balance,
		amount: Self::Balance,
		to: &AccountId,
	) -> DispatchResult;

	/// Expand the currency supply.
	fn expand_supply(
		currency_id: Self::CurrencyId,
		total_issuance: Self::Balance,
		amount: Self::Balance,
		to: &AccountId,
	) -> DispatchResult;

	/// Contracts or expands the currency supply based on conditions.
	fn on_block_with_price(block: BlockNumber, currency_id: Self::CurrencyId, price: Price) -> DispatchResult;

	/// Expands (if the price is high) or contracts (if the price is low) the currency supply.
	fn serp_elast(currency_id: Self::CurrencyId, price: Price) -> DispatchResult;

	/// Calculate the amount of supply change from a fraction given as `numerator` and `denominator`.
	fn supply_change(currency_id: Self::CurrencyId, numerator: u64, denominator: u64, supply: u64) -> u64;
	
}
