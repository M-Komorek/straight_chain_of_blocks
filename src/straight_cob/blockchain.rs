use log::{debug, info};
use std::collections::HashMap;

use super::{
    account::Account,
    block::Block,
    transactions::{Transaction, TransactionVariant},
};

#[derive(Debug)]
pub struct Blockchain<'a> {
    accounts: HashMap<String, Account>,
    blocks: Vec<Block<'a>>,
}

impl<'a> Blockchain<'a> {
    pub fn new(genesis_account: Account) -> Blockchain<'a> {
        debug!("Blockchain instance created");
        let mut accounts = HashMap::new();
        accounts.insert(genesis_account.user_name().into(), genesis_account);
        Blockchain {
            accounts,
            blocks: Vec::new(),
        }
    }

    pub fn append_block(&mut self, block: Block<'a>) -> Result<(), String> {
        if !block.verify() {
            return Err(format!("The block hash is invalid!"));
        }

        if block.get_transaction_count() == 0 {
            return Err(format!("The block does not contain any transaction!"));
        }

        block
            .get_transactions()
            .iter()
            .for_each(|transaction| self.execute_transaction(transaction));

        self.blocks.push(block);
        Ok(())
    }

    pub fn get_last_block_hahs(&self) -> &Option<Vec<u8>> {
        self.blocks.last().unwrap().get_hash()
    }

    pub fn get_account(&self, user_name: String) -> Result<&Account, String> {
        if let Some(account) = self.accounts.get(&user_name) {
            Ok(&account)
        } else {
            Err(format!("There is no account with provided username!"))
        }
    }

    fn execute_transaction(&mut self, transaction: &Transaction) {
        match transaction.get_transaction_variant() {
            TransactionVariant::CreateUserAccount { account_name } => {
                self.add_account(account_name)
            }
            TransactionVariant::CreateTokens {
                account_name,
                tokens,
            } => {
                if let Some(account) = self.accounts.get_mut(&String::from(*account_name)) {
                    account.add_tokens(*tokens);
                }
            }
            TransactionVariant::TransferTokens {
                sender_name,
                receiver_name,
                tokens,
            } => {
                if let Some(account) = self.accounts.get_mut(&String::from(*sender_name)) {
                    account.subtract_tokens(*tokens);
                }
                if let Some(account) = self.accounts.get_mut(&String::from(*receiver_name)) {
                    account.add_tokens(*tokens);
                }
            }
        }
    }

    fn add_account(&mut self, account_name: &str) {
        let account = Account::new(account_name);
        info!("New account has been created: {:?}", account);
        self.accounts.insert(String::from(account_name), account);
    }
}
