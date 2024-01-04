use blake2::{Blake2b512, Digest};
use chrono::{DateTime, Local};
use std::io::Write;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub enum TransactionVariant {
    CreateUserAccount {
        account_name: String,
    },
    CreateTokens {
        account_id: Uuid,
        tokens: u64,
    },
    TransferTokens {
        sender_id: Uuid,
        receiver_id: Uuid,
        tokens: u64,
    },
}

impl TransactionVariant {
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::new();

        match self {
            TransactionVariant::CreateUserAccount { account_name } => {
                buffer.write_all(account_name.as_bytes()).unwrap();
            }
            TransactionVariant::CreateTokens { account_id, tokens } => {
                buffer.write_all(&account_id.as_bytes().to_vec()).unwrap();
                buffer.write_all(&tokens.to_be_bytes()).unwrap();
            }
            TransactionVariant::TransferTokens {
                sender_id,
                receiver_id,
                tokens,
            } => {
                buffer.write_all(&sender_id.as_bytes().to_vec()).unwrap();
                buffer.write_all(&receiver_id.as_bytes().to_vec()).unwrap();
                buffer.write_all(&tokens.to_be_bytes()).unwrap();
            }
        }

        buffer
    }
}

#[derive(Debug)]
pub struct Transaction {
    creator_id: Uuid,
    creation_time: DateTime<Local>,
    variant: TransactionVariant,
}

impl Transaction {
    pub fn new(creator_id: Uuid, variant: TransactionVariant) -> Transaction {
        Transaction {
            creator_id,
            creation_time: Local::now(),
            variant,
        }
    }

    pub fn get_transaction_variant(&self) -> &TransactionVariant {
        &self.variant
    }

    pub fn calculate_hash(&self) -> Vec<u8> {
        let mut hasher = Blake2b512::new();

        hasher.update(self.creator_id.as_bytes());
        hasher.update(self.creation_time.to_rfc3339().as_bytes());
        hasher.update(self.variant.as_bytes());

        hasher.finalize().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transaction_variant_as_bytes_should_return_correct_value_for_create_user_account_variant() {
        let variant = TransactionVariant::CreateUserAccount {
            account_name: String::from("user"),
        };

        let expected_bytes = "user".as_bytes().to_vec();
        assert_eq!(variant.as_bytes(), expected_bytes);
    }

    #[test]
    fn transaction_variant_as_bytes_should_return_correct_value_for_create_tokens_variant() {
        let account_id = Uuid::new_v4();
        let tokens = 1000;

        let transaction = TransactionVariant::CreateTokens { account_id, tokens };

        let result = transaction.as_bytes();

        let mut expected_result = Vec::new();
        expected_result.extend_from_slice(&account_id.as_bytes().to_vec());
        expected_result.extend_from_slice(&tokens.to_be_bytes());

        assert_eq!(result, expected_result);
    }

    #[test]
    fn transaction_variant_as_bytesp_should_return_correct_value_for_transfer_tokens_variant() {
        let sender_id = Uuid::new_v4();
        let receiver_id = Uuid::new_v4();
        let tokens = 500;

        let transaction = TransactionVariant::TransferTokens {
            sender_id,
            receiver_id,
            tokens,
        };

        let result = transaction.as_bytes();

        let mut expected_result = Vec::new();
        expected_result.extend_from_slice(&sender_id.as_bytes().to_vec());
        expected_result.extend_from_slice(&receiver_id.as_bytes().to_vec());
        expected_result.extend_from_slice(&tokens.to_be_bytes());

        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_create_correct_transaction() {
        let creator_id = Uuid::new_v4();
        let variant = TransactionVariant::CreateUserAccount {
            account_name: String::from("user"),
        };

        let transaction = Transaction::new(creator_id, variant);
        assert_eq!(transaction.creator_id, creator_id);
        assert!(transaction.creation_time <= Local::now());
    }

    #[test]
    fn get_transaction_variant_should_return_correct_variant() {
        let creator_id = Uuid::new_v4();
        let receiver_account_id = Uuid::new_v4();
        let variant = TransactionVariant::CreateTokens {
            account_id: receiver_account_id,
            tokens: 100,
        };

        let transaction = Transaction::new(creator_id, variant);
        match transaction.get_transaction_variant() {
            TransactionVariant::CreateTokens { account_id, tokens } => {
                assert_eq!(*account_id, receiver_account_id);
                assert_eq!(*tokens, 100);
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn calculate_hash_should_return_expected_result() {
        let creator_id = Uuid::new_v4();
        let variant = TransactionVariant::TransferTokens {
            sender_id: Uuid::new_v4(),
            receiver_id: Uuid::new_v4(),
            tokens: 50,
        };
        let transaction = Transaction::new(creator_id, variant.clone());

        let mut hasher = Blake2b512::new();
        hasher.update(creator_id.as_bytes());
        hasher.update(transaction.creation_time.to_rfc3339().as_bytes());
        hasher.update(variant.as_bytes());

        let expected_hash = hasher.finalize().to_vec();
        assert_eq!(transaction.calculate_hash(), expected_hash);
    }
}
