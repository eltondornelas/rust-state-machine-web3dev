use support::Dispatch;

mod balances;
mod support;
mod system;

// use balances::Pallet;

mod types {
    pub type Balance = u128;
    pub type AccountId = String;
    pub type BlockNumber = u32;
    pub type Nonce = u32;

    pub type Extrinsic = crate::support::Extrinsic<AccountId, crate::RuntimeCall>;
    pub type Header = crate::support::Header<BlockNumber>;
    pub type Block = crate::support::Block<Header, Extrinsic>;
}

pub enum RuntimeCall {}

#[derive(Debug)]
pub struct Runtime {
    balances: balances::Pallet<Self>,
    system: system::Pallet<Self>,
}

impl system::Config for Runtime {
    type AccountId = types::AccountId;
    type BlockNumber = types::BlockNumber;
    type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
    type Balance = types::Balance;
}

impl Runtime {
    fn new() -> Self {
        Self {
            balances: balances::Pallet::new(),
            system: system::Pallet::new(),
        }
    }

    // Execute a block of extrinsics. Increments the block number.
    fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
        self.system.increment_block_number();

        if block.header.block_number != self.system.block_number() {
            return Err("Block number mismatch");
        }

        // iter: This method creates an iterator that borrows each element from the vector, allowing you to read the values without taking ownership. It's useful when you want to iterate over the vector while keeping it intact.
        // into_iter: This method consumes the vector, transferring ownership of each element to the iterator. It's handy when you want to move or transfer ownership of the vector's elements to another part of your code. After using into_iter, the original vector can't be used anymore, as ownership has been transferred.
        for (i, support::Extrinsic { caller, call }) in block.extrinsics.into_iter().enumerate() {
            self.system.increment_nonce(&caller);

            let res = self.dispatch(caller, call).map_err(|e| {
                format!(
                    "Error in block {}: extrinsic {}: {}",
                    block.header.block_number, i, e
                )
            });
        }

        /*
        * let _res = self.dispatch(caller, call).map_err(|e| eprintln!("{}", e));
        * eprintln!(
           "Extrinsic Error\n\tBlock Number: {}\n\tExtrinsic Number: {}\n\tError: {}",
           block.header.block_number, i, e
          )
        */

        Ok(())
    }
}

impl crate::support::Dispatch for Runtime {
    type Caller = <Runtime as system::Config>::AccountId;
    type Call = RuntimeCall;
    // Dispatch a call on behalf of a caller. Increments the caller's nonce.
    //
    // Dispatch allows us to identify which underlying module call we want to execute.
    // Note that we extract the `caller` from the extrinsic, and use that information
    // to determine who we are executing the call on behalf of.
    fn dispatch(
        &mut self,
        caller: Self::Caller,
        runtime_call: Self::Call,
    ) -> support::DispatchResult {
        unimplemented!();
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
