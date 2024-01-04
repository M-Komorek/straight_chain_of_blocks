mod straight_cob;

use log::{error, info};
use straight_cob::{
    account::Account,
    block::Block,
    blockchain::Blockchain,
    transactions::{Transaction, TransactionVariant},
};

fn main() {
    env_logger::init();

    let genesis_account = Account::new("Genesis");
    let genesis_account_id = genesis_account.id().clone();
    let mut straight_chain_of_blocks = Blockchain::new(genesis_account);

    // Create accounts
    let mut genesis_block = Block::new(None);

    ["Monica", "Chandler"].iter().for_each(|username| {
        genesis_block.append_transaction(Transaction::new(
            genesis_account_id,
            TransactionVariant::CreateUserAccount {
                account_name: String::from(*username),
            },
        ));
    });

    if let Err(err) = straight_chain_of_blocks.append_block(genesis_block) {
        error!("{}", err);
    } else {
        info!("New block has been added correctly");
    }

    info!(
        "Currecnt accounts state: {:#?}",
        straight_chain_of_blocks.get_accounts()
    );

    // Append tokens
    let mut create_tokens_block =
        Block::new(straight_chain_of_blocks.get_last_block_hahs().clone());
    let monica = straight_chain_of_blocks.get_account("Monica").unwrap();
    let chandler = straight_chain_of_blocks.get_account("Chandler").unwrap();

    [monica, chandler].iter().for_each(|account| {
        create_tokens_block.append_transaction(Transaction::new(
            genesis_account_id,
            TransactionVariant::CreateTokens {
                account_id: account.id().clone(),
                tokens: 100,
            },
        ));
    });

    if let Err(err) = straight_chain_of_blocks.append_block(create_tokens_block) {
        error!("{}", err);
    } else {
        info!("New block has been added correctly");
    }

    info!(
        "Currecnt accounts state: {:#?}",
        straight_chain_of_blocks.get_accounts()
    );

    // Transfer tokens
    let mut transfer_tokens_block =
        Block::new(straight_chain_of_blocks.get_last_block_hahs().clone());
    let monica = straight_chain_of_blocks.get_account("Monica").unwrap();
    let chandler = straight_chain_of_blocks.get_account("Chandler").unwrap();

    transfer_tokens_block.append_transaction(Transaction::new(
        genesis_account_id,
        TransactionVariant::TransferTokens {
            sender_id: monica.id().clone(),
            receiver_id: chandler.id().clone(),
            tokens: 1,
        },
    ));

    if let Err(err) = straight_chain_of_blocks.append_block(transfer_tokens_block) {
        error!("{}", err);
    } else {
        info!("New block has been added correctly");
    }

    info!(
        "Currecnt accounts state: {:#?}",
        straight_chain_of_blocks.get_accounts()
    );
}
