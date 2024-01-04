use std::collections::HashMap;
use uuid::Uuid;

use super::{
    account::Account,
    block::Block,
    transactions::{Transaction, TransactionVariant},
};

#[derive(Debug)]
pub struct Blockchain {
    accounts: HashMap<String, Account>,
    blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new(genesis_account: Account) -> Blockchain {
        let mut accounts = HashMap::new();
        accounts.insert(genesis_account.user_name().to_string(), genesis_account);

        Blockchain {
            accounts,
            blocks: Vec::new(),
        }
    }

    pub fn get_last_block_hahs(&self) -> &Option<Vec<u8>> {
        self.blocks.last().unwrap().get_hash()
    }

    pub fn get_account(&self, user_name: &str) -> Option<&Account> {
        if let Some(account) = self.accounts.get(user_name) {
            Some(&account)
        } else {
            None
        }
    }

    pub fn get_accounts(&self) -> &HashMap<String, Account> {
        &self.accounts
    }

    pub fn append_block(&mut self, block: Block) -> Result<(), String> {
        if !block.verify() {
            return Err(format!("The block hash is invalid!"));
        }

        if block.get_transaction_count() == 0 {
            return Err(format!("The block does not contain any transaction!"));
        }

        if !self.blocks.is_empty() && block.get_prev_hash() != self.get_last_block_hahs() {
            return Err(format!(
                "The new block does not point to the previous block!"
            ));
        }

        self.execute_transactions(&block)?;
        self.blocks.push(block);

        Ok(())
    }

    fn execute_transactions(&mut self, block: &Block) -> Result<(), String> {
        let current_accounts_state = self.accounts.clone();

        for transaction in block.get_transactions() {
            if let Err(err) = self.execute_transaction(transaction) {
                self.accounts = current_accounts_state;
                return Err(format!(
                    "Could not execute transaction due to an error: {}.",
                    err
                ));
            }
        }

        Ok(())
    }

    fn execute_transaction(&mut self, transaction: &Transaction) -> Result<(), String> {
        match transaction.get_transaction_variant() {
            TransactionVariant::CreateUserAccount { account_name } => {
                self.create_user_account(account_name)
            }
            TransactionVariant::CreateTokens { account_id, tokens } => {
                self.create_tokens(account_id, *tokens)
            }
            TransactionVariant::TransferTokens {
                sender_id,
                receiver_id,
                tokens,
            } => self.transfer_tokens(sender_id, receiver_id, *tokens),
        }
    }

    fn create_user_account(&mut self, account_name: &str) -> Result<(), String> {
        self.accounts
            .insert(account_name.to_string(), Account::new(account_name));
        Ok(())
    }

    fn create_tokens(&mut self, account_id: &Uuid, tokens: u64) -> Result<(), String> {
        if let Some(account) = self.get_account_by_id_mut(account_id) {
            account.add_tokens(tokens);
            Ok(())
        } else {
            Err(format!("Create tokens transaction failed!"))
        }
    }

    fn transfer_tokens(
        &mut self,
        sender_id: &Uuid,
        receiver_id: &Uuid,
        tokens: u64,
    ) -> Result<(), String> {
        if let Some(account) = self.get_account_by_id_mut(sender_id) {
            account.subtract_tokens(tokens);
        }
        if let Some(account) = self.get_account_by_id_mut(receiver_id) {
            account.add_tokens(tokens);
        }
        Ok(())
    }

    fn get_account_by_id_mut(&mut self, account_id: &Uuid) -> Option<&mut Account> {
        if let Some((_, account)) = self
            .accounts
            .iter_mut()
            .find(|(_, acc)| acc.id() == account_id)
        {
            Some(account)
        } else {
            None
        }
    }
}
