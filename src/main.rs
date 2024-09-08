mod balances;
mod system;

// use balances::Pallet;

mod types {
    pub type Balance = u128;
    pub type AccountId = String;
}

#[derive(Debug)]
pub struct Runtime {
    balances: balances::Pallet<types::AccountId, types::Balance>,
    system: system::Pallet,
}

impl Runtime {
    fn new() -> Self {
        Self {
            balances: balances::Pallet::new(),
            system: system::Pallet::new(),
        }
    }
}

fn main() {
    let mut runtime = Runtime::new();
    let alice = "alice".to_string();
    let bob = "bob".to_string();
    let charlie = "charlie".to_string();

    runtime.balances.set_balance(alice.clone(), 100);
    runtime.system.increment_block_number();
    assert_eq!(runtime.system.block_number(), 1);

    runtime.system.increment_nonce(&alice);
    let _res = runtime
        .balances
        .transfer(alice.clone(), bob, 30)
        .map_err(|e| eprintln!("{}", e));

    runtime.system.increment_nonce(&alice);
    let _res = runtime
        .balances
        .transfer(alice, charlie, 20)
        .map_err(|e| eprintln!("{}", e));

    println!("{:#?}", runtime);
}

// cargo build
// cargo run -> compila e gera um binario executavel com mesmo nome do projeto; ./target/debug/hello
// cargo fmt
// touch src/balances.rs

/*
 * ctrl+j ou ctrl+`
 */
