mod straight_cob;

use log::info;
use straight_cob::{
    account::Account,
    block::Block,
    blockchain::Blockchain,
    transactions::{Transaction, TransactionVariant},
};

fn main() {
    env_logger::init();

    let genesis_account = Account::new("Genesis");
    let mut genesis_block = Block::new(None);

    ["Monica", "Chandler"].iter().for_each(|username| {
        genesis_block.append_transaction(Transaction::new(
            genesis_account.id().clone(),
            TransactionVariant::CreateUserAccount {
                account_name: username,
            },
        ));
        genesis_block.append_transaction(Transaction::new(
            genesis_account.id().clone(),
            TransactionVariant::CreateTokens {
                account_name: username,
                tokens: 1000,
            },
        ));
    });

    let mut straight_chain_of_blocks = Blockchain::new(genesis_account);
    straight_chain_of_blocks
        .append_block(genesis_block)
        .unwrap();

    info!("{:#?}", straight_chain_of_blocks);

    // handle append_block, execute transaction do some checks etc
    let mut another_block = Block::new(straight_chain_of_blocks.get_last_block_hahs().clone());
    another_block.append_transaction(Transaction::new(
        straight_chain_of_blocks
            .get_account(String::from("Genesis"))
            .unwrap()
            .id()
            .clone(),
        TransactionVariant::TransferTokens {
            sender_name: "Monica",
            receiver_name: "Chandler",
            tokens: 100,
        },
    ));

    straight_chain_of_blocks
        .append_block(another_block)
        .unwrap();

    info!("{:#?}", straight_chain_of_blocks);
}
