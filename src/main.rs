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

    pub type Content = String; // TODO: no roteiro seria &'static str; mas o correto mesmo aqui deveria ser um hash
}

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
            },
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
            },
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

    assert_eq!(
        runtime
            .proof_of_existence
            .get_claim(&"my_document2".to_string()),
        Some(&bob)
    );
    println!("{:#?}", runtime)
}
