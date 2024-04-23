/* TODO: You might need to update your imports. */
use core::ops::AddAssign;
use num::traits::{One, Zero};
use std::{collections::BTreeMap};
/// This is the System Pallet.
/// It handles low level state needed for your blockchain.

pub trait Config {
	type AccountId: Ord + Clone;
	type BlockNumber: Zero + One + AddAssign + Copy;
	type Nonce: Zero + One + Copy;
}
#[derive(Debug)]
pub struct Pallet<T: Config> {
	/// The current block number.
	/* TODO: Create a field `block_number` that stores a `u32`. */
	/// A map from an account to their nonce.
	/* TODO: Create a field `nonce` that is a `BTreeMap` from `String` to `u32`. */
	block_number: T::BlockNumber,
	nonce: BTreeMap<T::AccountId, T::Nonce>
}

impl<T: Config> Pallet <T>

{
	/// Create a new instance of the System Pallet.
	pub fn new() -> Self {
		/* TODO: Return a new instance of the `Pallet` struct. */
		Self {block_number:T::BlockNumber::zero(), nonce: BTreeMap::new()}
	}

	pub fn block_number(&self) -> T::BlockNumber {
		self.block_number
	}

	pub fn inc_block_number(&mut self) {
		self.block_number = self.block_number + T::BlockNumber::one();
	}

	pub fn inc_nonce(&mut self, who: &T::AccountId) {
		self.nonce.insert(who.clone(), *self.nonce.get(who).unwrap_or(&T::Nonce::zero())+T::Nonce::one());
	}

}
