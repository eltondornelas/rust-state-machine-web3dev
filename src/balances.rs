use crate::support::DispatchResult;
use num::traits::{CheckedAdd, CheckedSub, Zero};
use std::collections::BTreeMap;

/* In the Polkadot SDK ecosystem, we call this "tight coupling" because a runtime which contains the Balances Pallet must also contain the System Pallet.
   In a sense these two pallets are tightly coupled to one another.
*/
pub trait Config: crate::system::Config {
    type Balance: CheckedAdd + CheckedSub + Zero + Copy;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    balances: BTreeMap<T::AccountId, T::Balance>,
}

#[macros::call]
impl<T: Config> Pallet<T> {
    /// Transfere `amount` de uma conta para outra.
    /// Esta função verifica se `caller` tem pelo menos `amount` de saldo para transferir,
    /// e se não ocorrem overflow/underflow matemáticos.
    pub fn transfer(
        &mut self,
        caller: T::AccountId,
        to: T::AccountId,
        amount: T::Balance,
    ) -> DispatchResult {
        let caller_balance = self.get_balance(caller.clone());
        let to_balance = self.get_balance(to.clone());

        let new_caller_balance = caller_balance
            .checked_sub(&amount)
            .ok_or("Insufficient balance")?; // Underflow

        let new_to_balance = to_balance.checked_add(&amount).ok_or("Overflow")?;

        self.balances.insert(caller, new_caller_balance);
        self.balances.insert(to, new_to_balance);

        Ok(())
    }
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Pallet {
            balances: BTreeMap::new(),
        }
    }

    pub fn set_balance(&mut self, account: T::AccountId, amount: T::Balance) {
        self.balances.insert(account, amount);
    }

    /*
        Note que fazemos nosso pequeno truque aqui!
        Em vez de expor uma API que força o usuário a lidar com um Option,
        somos capazes de fazer nossa API sempre retornar um u128 convertendo qualquer usuário com valor None em 0.
    */
    pub fn get_balance(&self, account: T::AccountId) -> T::Balance {
        *self.balances.get(&account).unwrap_or(&T::Balance::zero())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestConfig;

    impl super::Config for TestConfig {
        type Balance = u128;
    }

    impl crate::system::Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    #[test]
    fn init_balances() {
        let mut balances = super::Pallet::<TestConfig>::new();

        assert_eq!(balances.get_balance("alice".to_string()), 0);
        balances.set_balance("alice".to_string(), 100);
        assert_eq!(balances.get_balance("alice".to_string()), 100);
        assert_eq!(balances.get_balance("bob".to_string()), 0);
    }

    #[test]
    fn transfer_balance() {
        let mut balances = Pallet::<TestConfig>::new();

        assert_eq!(
            balances.transfer("daniel".to_string(), "vini".to_string(), 10),
            Err("Insufficient balance")
        );

        balances.set_balance("daniel".to_string(), 10);
        assert_eq!(
            balances.transfer("daniel".to_string(), "vini".to_string(), 3),
            Ok(())
        );

        assert_eq!(balances.get_balance("daniel".to_string()), 7);
        assert_eq!(balances.get_balance("vini".to_string()), 3);

        balances.set_balance("vini".to_string(), u128::MAX);
        assert_eq!(
            balances.transfer("daniel".to_string(), "vini".to_string(), 3),
            Err("Overflow")
        );
    }
}
