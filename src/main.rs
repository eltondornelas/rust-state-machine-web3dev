use crate::balances::Call::transfer;
use proof_of_existence::Call::{create_claim, revoke_claim};
use support::Dispatch;

mod balances;
mod proof_of_existence;
mod support;
mod system;

mod types {
    pub type Balance = u128;
    pub type AccountId = String;
    pub type BlockNumber = u32;
    pub type Nonce = u32;

    pub type Extrinsic = crate::support::Extrinsic<AccountId, crate::RuntimeCall>;
    pub type Header = crate::support::Header<BlockNumber>;
    pub type Block = crate::support::Block<Header, Extrinsic>;

    pub type Content = String; // todo: no roteiro seria &'static str; mas o correto mesmo aqui deveria ser um hash
}

// "outer enum"
// pub enum RuntimeCall {
//     Balances(balances::Call<Runtime>),
//     ProofOfExistence(proof_of_existence::Call<Runtime>),
// }

#[derive(Debug)]
#[macros::runtime]
pub struct Runtime {
    system: system::Pallet<Self>,
    balances: balances::Pallet<Self>,
    proof_of_existence: proof_of_existence::Pallet<Self>,
}

impl system::Config for Runtime {
    type AccountId = types::AccountId;
    type BlockNumber = types::BlockNumber;
    type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
    type Balance = types::Balance;
}

impl proof_of_existence::Config for Runtime {
    type Content = types::Content;
}

// impl Runtime {
//     fn new() -> Self {
//         Self {
//             balances: balances::Pallet::new(),
//             system: system::Pallet::new(),
//             proof_of_existence: proof_of_existence::Pallet::new(),
//         }
//     }

//     // Execute a block of extrinsics. Increments the block number.
//     fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
//         self.system.increment_block_number();

//         if block.header.block_number != self.system.block_number() {
//             return Err("Block number mismatch");
//         }

//         // iter: This method creates an iterator that borrows each element from the vector, allowing you to read the values without taking ownership. It's useful when you want to iterate over the vector while keeping it intact.
//         // into_iter: This method consumes the vector, transferring ownership of each element to the iterator. It's handy when you want to move or transfer ownership of the vector's elements to another part of your code. After using into_iter, the original vector can't be used anymore, as ownership has been transferred.
//         /*
//            An extrinsic error is not enough to trigger the block to be invalid. We capture the
//            result, and emit an error message if one is emitted.
//         */
//         for (i, support::Extrinsic { caller, call }) in block.extrinsics.into_iter().enumerate() {
//             self.system.increment_nonce(&caller);

//             let _res = self.dispatch(caller, call).map_err(|e| {
//                 eprintln!(
//                     "Extrinsic Error\n\tBlock Number: {}\n\tExtrinsic Number: {}\n\tError: {}",
//                     // "Error in block {}: extrinsic {}: {}",
//                     block.header.block_number,
//                     i,
//                     e
//                 )
//             });
//         }

//         Ok(())
//     }
// }

// impl crate::support::Dispatch for Runtime {
//     type Caller = <Runtime as system::Config>::AccountId;
//     type Call = RuntimeCall;
//     // Dispatch a call on behalf of a caller. Increments the caller's nonce.
//     //
//     // Dispatch allows us to identify which underlying module call we want to execute.
//     // Note that we extract the `caller` from the extrinsic, and use that information
//     // to determine who we are executing the call on behalf of.
//     fn dispatch(
//         &mut self,
//         caller: Self::Caller,
//         runtime_call: Self::Call,
//     ) -> support::DispatchResult {
//         match runtime_call {
//             RuntimeCall::Balances(call) => self.balances.dispatch(caller, call),
//             RuntimeCall::ProofOfExistence(call) => self.proof_of_existence.dispatch(caller, call),
//         }
//     }
// }

fn main() {
    let mut runtime = Runtime::new();
    let alice = "alice".to_string();
    let bob = "bob".to_string();
    let charlie = "charlie".to_string();

    runtime.balances.set_balance(alice.clone(), 100);

    let block_1 = types::Block {
        header: support::Header { block_number: 1 },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::balances(transfer {
                    to: bob.clone(),
                    amount: 30,
                }),
            },
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::balances(transfer {
                    to: charlie,
                    amount: 20,
                }),
            }, /*
               support::Extrinsic {
                   caller: alice.clone(),
                   call: RuntimeCall::Balances(SetBalance { value: 0 }),
               },
               support::Extrinsic {
                   caller: alice,
                   call: RuntimeCall::ProofOfExistence(create_claim {
                       claim: "my_content".to_string(),
                   }),
               },*/
        ],
    };

    let block_2 = types::Block {
        header: support::Header { block_number: 2 },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::proof_of_existence(create_claim {
                    claim: "my_document".to_string(),
                }),
            },
            support::Extrinsic {
                caller: bob.clone(),
                call: RuntimeCall::proof_of_existence(create_claim {
                    claim: "my_document2".to_string(),
                }),
            }, /*,
               support::Extrinsic {
                   caller: bob.clone(),
                   call: RuntimeCall::proof_of_existence(create_claim { claim: "my_document".to_string() }),
               }*/
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::proof_of_existence(revoke_claim {
                    claim: "my_document".to_string(),
                }),
            },
        ],
    };

    runtime.execute_block(block_1).expect("invalid block!");
    runtime.execute_block(block_2).expect("invalid block!");

    println!("{:#?}", runtime)
}

// cargo build
// cargo run -> compila e gera um binario executavel com mesmo nome do projeto; ./target/debug/hello
// cargo fmt
// touch src/balances.rs

/*
 * ctrl+j ou ctrl+`
 */
