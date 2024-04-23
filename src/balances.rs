use std::collections::BTreeMap;
use num::traits::{CheckedAdd, CheckedSub, Zero};

/// This is the Balances Module.
/// It is a simple module which keeps track of how much balance each account has in this state
/// machine.

pub trait Config: crate::system::Config {
	type Balance: CheckedAdd + CheckedSub + Zero + Copy;
}
#[derive(Debug)]
pub struct Pallet<T: Config> {
	// A simple storage mapping from accounts (`String`) to their balances (`u128`).
	balances: BTreeMap<T::AccountId, T::Balance>,
}

impl<T: Config> Pallet<T> 
{
	/// Create a new instance of the balances module.
	pub fn new() -> Self {
		Self { balances: BTreeMap::new() }
	}

	/// Set the balance of an account `who` to some `amount`.
	pub fn set_balance(&mut self, who: &T::AccountId, amount: T::Balance) {
		/* Insert `amount` into the BTreeMap under `who`. */
		self.balances.insert(who.clone(), amount);
	}

	/// Get the balance of an account `who`.
	/// If the account has no stored balance, we return zero.
	pub fn balance(&self, who: &T::AccountId) -> T::Balance {
		/* Return the balance of `who`, returning zero if `None`. */
		*self.balances.get(who).unwrap_or(&T::Balance::zero())
	}
    pub fn transfer(
		&mut self,
		caller: T::AccountId,
		to: T::AccountId,
		amount: T::Balance,
	) -> crate::support::DispatchResult {
		/* TODO:
			- Get the balance of account `caller`.
			- Get the balance of account `to`.

			- Use safe math to calculate a `new_caller_balance`.
			- Use safe math to calculate a `new_to_balance`.

			- Insert the new balance of `caller`.
			- Insert the new balance of `to`.
		*/

        let balance_caller = self.balance(&caller);
        let balance_to = self.balance(&to);
        let new_caller_balance = balance_caller.checked_sub(&amount).ok_or("Not enough funds")?;
        let new_to_balance = balance_to.checked_add(&amount).ok_or("Overflow - max fund limit reached")?;
        self.set_balance(&caller, new_caller_balance);
        self.set_balance(&to,new_to_balance);
		Ok(())
	}
}

pub enum Call<T: Config> {
	/* TODO: Create an enum variant `Transfer` which contains named fields:
		- `to`: a `T::AccountId`
		- `amount`: a `T::Balance`
	*/
	Transfer {
		to: T::AccountId,
		amount: T::Balance
	}
}

/// Implementation of the dispatch logic, mapping from `BalancesCall` to the appropriate underlying
/// function we want to execute.
impl<T: Config> crate::support::Dispatch for Pallet<T> {
	type Caller = T::AccountId;
	type Call = Call<T>;

	fn dispatch(
		&mut self,
		caller: Self::Caller,
		call: Self::Call,
	) -> crate::support::DispatchResult {
		/* TODO: use a `match` statement to route the `Call` to the appropriate pallet function. */
		match call {
			Call::Transfer { to, amount } =>
				self.transfer(caller, to, amount),
			_ => Ok(())
		}
	}
}

#[cfg(test)]
mod tests {
    #[test]
    fn init_balances() {

		pub struct TestConfig{}
		impl super::Config for TestConfig {
			type Balance = u128;
		}
		impl crate::system::Config for TestConfig {
			type AccountId =  String;
			type BlockNumber = u32;
			type Nonce = u32;
		}
        let mut balances = super::Pallet::<TestConfig>::new();

        assert_eq!(balances.balance(&"alice".to_string()), 0);
        balances.set_balance(&"alice".to_string(), 100);
        assert_eq!(balances.balance(&"alice".to_string()), 100);
        assert_eq!(balances.balance(&"bob".to_string()), 0);
    }

    #[test]
	fn transfer_balance() {
		pub struct TestConfig{}
		impl super::Config for TestConfig {
			type Balance = u128;
		}
		impl crate::system::Config for TestConfig {
			type AccountId =  String;
			type BlockNumber = u32;
			type Nonce = u32;
		}
        let mut balances = super::Pallet::<TestConfig>::new();

        assert_eq!(balances.balance(&"alice".to_string()), 0);
        assert_eq!(balances.transfer("alice".to_string(),"bob".to_string(),50),Err("Not enough funds"));
        balances.set_balance(&"alice".to_string(), 100);
        balances.transfer("alice".to_string(),"bob".to_string(),50);
        assert_eq!(balances.balance(&"alice".to_string()), 50);
        assert_eq!(balances.balance(&"alice".to_string()), 50);

	}
}

