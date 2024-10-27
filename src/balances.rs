use num::traits::{CheckedAdd, CheckedSub, Zero};
use std::collections::BTreeMap;

// type Balance = u128;
// type AccountId = String;
// tipo abstrato; ao utilizar generico na no Pallet, fica desnecessario a inclusao do "type" aqui no arquivo

pub trait Config {
    type AccountId: Ord + Clone;
    type Balance: CheckedAdd + CheckedSub + Zero + Copy;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    balances: BTreeMap<T::AccountId, T::Balance>,
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Pallet {
            balances: BTreeMap::new(),
        }
    }

    // video
    pub fn set_balance(&mut self, account: T::AccountId, amount: T::Balance) {
        self.balances.insert(account, amount);
    }

    // escrita
    // pub fn set_balance(&mut self, who: &String, amount: u128) {
    //     self.balances.insert(who.clone(), amount);
    // }

    /*
        Note que fazemos nosso pequeno truque aqui!
        Em vez de expor uma API que força o usuário a lidar com um Option,
        somos capazes de fazer nossa API sempre retornar um u128 convertendo qualquer usuário com valor None em 0.
    */
    // codigo no video
    pub fn get_balance(&self, account: T::AccountId) -> T::Balance {
        *self.balances.get(&account).unwrap_or(&T::Balance::zero())
        // match self.balances.get(&account) {
        //     Some(amount) => *amount,
        //     None => 0
        // }
    }

    // codigo escrito no texto
    // pub fn balance(&self, who: &String) -> u128 {
    //     *self.balances.get(&who).unwrap_or(&0)
    // }

    // pub fn balance(&self, account: &String) -> u128 {
    //     *self.balances.get(account).unwrap_or(&0)
    // }

    /// Transfere `amount` de uma conta para outra.
    /// Esta função verifica se `caller` tem pelo menos `amount` de saldo para transferir,
    /// e se não ocorrem overflow/underflow matemáticos.
    pub fn transfer(
        &mut self,
        caller: T::AccountId,
        to: T::AccountId,
        amount: T::Balance,
    ) -> Result<(), &'static str> {
        let caller_balance = self.get_balance(caller.clone());
        let to_balance = self.get_balance(to.clone());

        let new_caller_balance = caller_balance
            .checked_sub(&amount)
            .ok_or("Insufficient balance")?; // Underflow

        let new_to_balance = to_balance.checked_add(&amount).ok_or("Overflow")?; // Overflow, caso estoure o valor maximo do tipo

        self.balances.insert(caller, new_caller_balance);
        self.balances.insert(to, new_to_balance);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestConfig;

    impl super::Config for TestConfig {
        type AccountId = String;
        type Balance = u128;
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
