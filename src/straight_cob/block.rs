use blake2::{Blake2b512, Digest};

use super::transactions::Transaction;

#[derive(Debug)]
pub struct Block<'a> {
    hash: Option<Vec<u8>>,
    prev_hash: Option<Vec<u8>>,
    transactions: Vec<Transaction<'a>>,
}

impl<'a> Block<'a> {
    pub fn new(prev_hash: Option<Vec<u8>>) -> Block<'a> {
        Block {
            hash: None,
            prev_hash,
            transactions: Vec::new(),
        }
    }

    pub fn get_hash(&self) -> &Option<Vec<u8>> {
        &self.hash
    }

    pub fn get_transaction_count(&self) -> usize {
        self.transactions.len()
    }

    pub fn get_transactions(&self) -> &Vec<Transaction<'a>> {
        &self.transactions
    }

    pub fn append_transaction(&mut self, transaction: Transaction<'a>) {
        self.transactions.push(transaction);
        self.update_hash();
    }

    pub fn verify(&self) -> bool {
        if let Some(hash) = &self.hash {
            return hash == &self.calculate_hash();
        } else {
            return false;
        }
    }

    fn update_hash(&mut self) {
        self.hash = Some(self.calculate_hash());
    }

    fn calculate_hash(&self) -> Vec<u8> {
        let mut hasher = Blake2b512::new();

        if let Some(prev_hash) = &self.prev_hash {
            hasher.update(prev_hash);
        }

        self.transactions
            .iter()
            .for_each(|transaction| hasher.update(transaction.calculate_hash()));

        hasher.finalize().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::straight_cob::transactions::TransactionVariant;

    use super::*;

    #[test]
    fn should_create_block_without_prev_hash() {
        let block = Block::new(None);

        assert_eq!(block.hash, None);
        assert_eq!(block.prev_hash, None);
        assert!(block.transactions.is_empty());
    }

    #[test]
    fn should_create_block_with_prev_hash() {
        let prev_hash = Some(vec![1, 2, 3, 4]);
        let block = Block::new(prev_hash.clone());

        assert_eq!(block.hash, None);
        assert_eq!(block.prev_hash, prev_hash);
        assert!(block.transactions.is_empty());
    }

    #[test]
    fn append_transaction_should_increase_transaction_count_and_update_hash() {
        let mut block = Block::new(None);
        let transaction = Transaction::new(
            Uuid::new_v4(),
            TransactionVariant::CreateUserAccount {
                account_name: "account",
            },
        );
        block.append_transaction(transaction);

        assert_eq!(block.get_transaction_count(), 1);
        assert_ne!(block.get_hash(), &None);
    }

    #[test]
    fn verify_should_return_true_for_valid_hash() {
        let mut block = Block::new(None);
        let transaction = Transaction::new(
            Uuid::new_v4(),
            TransactionVariant::CreateUserAccount {
                account_name: "account",
            },
        );
        block.append_transaction(transaction);

        assert!(block.verify());
    }

    #[test]
    fn verify_should_return_false_for_invalid_hash() {
        let mut block = Block::new(None);
        let transaction = Transaction::new(
            Uuid::new_v4(),
            TransactionVariant::CreateUserAccount {
                account_name: "account",
            },
        );
        block.append_transaction(transaction);

        if let Some(hash) = &mut block.hash {
            hash[0] ^= 1;
        }

        assert!(!block.verify());
    }

    #[test]
    fn calculate_hash_should_produce_non_empty_hash() {
        let mut block = Block::new(None);
        let transaction = Transaction::new(
            Uuid::new_v4(),
            TransactionVariant::CreateUserAccount {
                account_name: "account",
            },
        );
        block.append_transaction(transaction);

        let hash = block.calculate_hash();
        assert_ne!(hash, vec![]);
    }
}
