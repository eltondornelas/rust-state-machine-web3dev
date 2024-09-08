use num::traits::{One, Zero};
use std::collections::BTreeMap;

// type BlockNumber = u32;
// type Nonce = u32;
// type AccountId = String;
// tipos abstratos

// module for blockchain metadata
#[derive(Debug)]
pub struct Pallet<BlockNumber, AccountId, Nonce> {
    block_number: BlockNumber,
    nonce: BTreeMap<AccountId, Nonce>,
}

impl<BlockNumber, AccountId, Nonce> Pallet<BlockNumber, AccountId, Nonce>
where
    BlockNumber: Zero + One + Copy,
    AccountId: Ord + Clone,
    Nonce: Zero + Copy + One,
{
    pub fn new() -> Self {
        Pallet {
            block_number: BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }

    pub fn block_number(&self) -> BlockNumber {
        self.block_number
    }

    pub fn increment_block_number(&mut self) {
        self.block_number = self.block_number + BlockNumber::one();
    }

    pub fn increment_nonce(&mut self, account: &AccountId) {
        let nonce = *self.nonce.get(account).unwrap_or(&Nonce::zero()) + Nonce::one();
        self.nonce.insert(account.clone(), nonce);
    }
}

#[cfg(test)]
mod test {
    use crate::system::Pallet;

    #[test]
    fn init_system() {
        let mut system = Pallet::<u64, String, u32>::new();

        assert_eq!(system.block_number, 0);
        assert_eq!(system.nonce.get("daniel"), None);

        system.increment_block_number();
        assert_eq!(system.block_number(), 1);

        system.increment_nonce(&"daniel".to_string());
        assert_eq!(system.nonce.get("daniel").unwrap(), &1);
    }
}
