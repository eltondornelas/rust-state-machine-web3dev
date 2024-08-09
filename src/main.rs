mod balances;
// use balances::Pallet;

fn main() {
    let mut pallet = balances::Pallet::new();
    pallet.set_balance("daniel".to_string(), 2);

    let balance = pallet.get_balance("daniel".to_string());
    println!("Balance: {}", balance);
    println!("Hello, web3devs!!")
}

// cargo build
// cargo run -> compila e gera um binario executavel com mesmo nome do projeto; ./target/debug/hello
// cargo fmt
// touch src/balances.rs

/*
 * ctrl+j ou ctrl+`
 */
