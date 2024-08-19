use std::collections::BTreeMap;

// module for blockchain metadata
#[derive(Debug)]
pub struct Pallet {
    block_number: u32,
    nonce: BTreeMap<String, u32>,
}

impl Pallet {
    pub fn new() -> Self {
        Pallet {
            block_number: 0,
            nonce: BTreeMap::new(),
        }
    }

    pub fn block_number(&self) -> u32 {
        self.block_number
    }

    pub fn increment_block_number(&mut self) {
        self.block_number += 1; // nao eh safe math; o block comeca do 0 e o valor maximo nao vai chegar por isso
    }

    pub fn increment_nonce(&mut self, account: &String) {
        let nonce = self.nonce.get(account).unwrap_or(&0) + 1;
        self.nonce.insert(account.clone(), nonce);
    }
}

#[cfg(test)]
mod test {
    use crate::system::Pallet;

    #[test]
    fn init_system() {
        let mut system = Pallet::new();

        assert_eq!(system.block_number, 0);
        assert_eq!(system.nonce.get("daniel"), None);

        system.increment_block_number();
        assert_eq!(system.block_number(), 1);

        system.increment_nonce(&"daniel".to_string());
        assert_eq!(system.nonce.get("daniel").unwrap(), &1);
    }
}
