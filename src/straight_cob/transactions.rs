use blake2::{Blake2b512, Digest};
use chrono::{DateTime, Local};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub enum TransactionVariant<'a> {
    CreateUserAccount {
        account_name: &'a str,
    },
    CreateTokens {
        account_name: &'a str,
        tokens: u64,
    },
    TransferTokens {
        sender_name: &'a str,
        receiver_name: &'a str,
        tokens: u64,
    },
}

impl<'a> TransactionVariant<'a> {
    pub fn as_bytes(&self) -> Vec<u8> {
        match self {
            TransactionVariant::CreateUserAccount { account_name } => {
                [account_name.as_bytes()].concat()
            }
            TransactionVariant::CreateTokens {
                account_name,
                tokens,
            } => [account_name.as_bytes(), &tokens.to_ne_bytes()].concat(),
            TransactionVariant::TransferTokens {
                sender_name,
                receiver_name,
                tokens,
            } => [
                sender_name.as_bytes(),
                receiver_name.as_bytes(),
                &tokens.to_ne_bytes(),
            ]
            .concat(),
        }
    }
}

#[derive(Debug)]
pub struct Transaction<'a> {
    creator_id: Uuid,
    creation_time: DateTime<Local>,
    variant: TransactionVariant<'a>,
}

impl<'a> Transaction<'a> {
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
            account_name: "user",
        };

        let expected_bytes = "user".as_bytes().to_vec();
        assert_eq!(variant.as_bytes(), expected_bytes);
    }

    #[test]
    fn transaction_variant_as_bytes_should_return_correct_value_for_create_tokens_variant() {
        let variant = TransactionVariant::CreateTokens {
            account_name: "user",
            tokens: 100,
        };

        let expected_bytes = ["user".as_bytes(), &100u64.to_ne_bytes()].concat();
        assert_eq!(variant.as_bytes(), expected_bytes);
    }

    #[test]
    fn transaction_variant_as_bytesp_should_return_correct_value_for_transfer_tokens_variant() {
        let variant = TransactionVariant::TransferTokens {
            sender_name: "sender",
            receiver_name: "receiver",
            tokens: 50,
        };

        let expected_bytes = [
            "sender".as_bytes(),
            "receiver".as_bytes(),
            &50u64.to_ne_bytes(),
        ]
        .concat();
        assert_eq!(variant.as_bytes(), expected_bytes);
    }

    #[test]
    fn should_create_correct_transaction() {
        let creator_id = Uuid::new_v4();
        let variant = TransactionVariant::CreateUserAccount {
            account_name: "user",
        };

        let transaction = Transaction::new(creator_id, variant);
        assert_eq!(transaction.creator_id, creator_id);
        assert!(transaction.creation_time <= Local::now());
    }

    #[test]
    fn get_transaction_variant_should_return_correct_variant() {
        let creator_id = Uuid::new_v4();
        let variant = TransactionVariant::CreateTokens {
            account_name: "user",
            tokens: 100,
        };

        let transaction = Transaction::new(creator_id, variant);
        match transaction.get_transaction_variant() {
            TransactionVariant::CreateTokens {
                account_name,
                tokens,
            } => {
                assert_eq!(*account_name, "user");
                assert_eq!(*tokens, 100);
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn calculate_hash_should_return_expected_result() {
        let creator_id = Uuid::new_v4();
        let variant = TransactionVariant::TransferTokens {
            sender_name: "sender",
            receiver_name: "receiver",
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
