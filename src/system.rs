use num::traits::{One, Zero};
use std::collections::BTreeMap;

pub trait Config {
    type AccountId: Ord + Clone;
    type BlockNumber: Zero + One + Copy; // AddAssign
    type Nonce: Zero + Copy + One;
}

/// module for blockchain metadata
/// This is the System Pallet
/// It handles low level state needed for your blockchain
#[derive(Debug)]
pub struct Pallet<T: Config> {
    block_number: T::BlockNumber,
    nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Pallet {
            block_number: T::BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }

    pub fn block_number(&self) -> T::BlockNumber {
        self.block_number
    }

    pub fn inc_block_number(&mut self) {
        self.block_number = self.block_number + T::BlockNumber::one();
    }

    pub fn inc_nonce(&mut self, account: &T::AccountId) {
        let nonce = *self.nonce.get(account).unwrap_or(&T::Nonce::zero()) + T::Nonce::one();
        self.nonce.insert(account.clone(), nonce);
    }
}

#[cfg(test)]
mod test {
    use crate::system::Pallet;

    struct TestConfig;

    impl super::Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    #[test]
    fn init_system() {
        let mut system = Pallet::<TestConfig>::new();

        assert_eq!(system.block_number, 0);
        assert_eq!(system.nonce.get("daniel"), None);

        system.inc_block_number();
        assert_eq!(system.block_number(), 1);

        system.inc_nonce(&"daniel".to_string());
        assert_eq!(system.nonce.get("daniel").unwrap(), &1);
    }
}
