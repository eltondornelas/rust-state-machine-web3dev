use std::collections::BTreeMap;

pub struct Pallet {
    balances: BTreeMap<String, u128>,
}

impl Pallet {
    pub fn new() -> Self {
        Pallet {
            balances: BTreeMap::new(),
        }
    }

    // video
    pub fn set_balance(&mut self, account: String, amount: u128) {
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
    // video
    pub fn get_balance(&self, account: String) -> u128 {
        *self.balances.get(&account).unwrap_or(&0)
        // match self.balances.get(&account) {
        //     Some(amount) => *amount,
        //     None => 0
        // }
    }

    // escrita
    // pub fn balance(&self, who: &String) -> u128 {
    //     *self.balances.get(&who).unwrap_or(&0)
    // }

    // pub fn balance(&self, account: &String) -> u128 {
    //     *self.balances.get(account).unwrap_or(&0)
    // }
}

#[test]
fn init_balances() {
    let mut balances = Pallet::new();

    assert_eq!(balances.get_balance("alice".to_string()), 0);
    balances.set_balance("alice".to_string(), 100);
    assert_eq!(balances.get_balance("alice".to_string()), 100);
    assert_eq!(balances.get_balance("bob".to_string()), 0);
}
