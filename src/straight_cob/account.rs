use uuid::Uuid;

#[derive(Debug)]
pub struct Account {
    id: Uuid,
    user_name: String,
    tokens: i64,
}

impl Account {
    pub fn new(user_name: &str) -> Account {
        Account {
            id: Uuid::new_v4(),
            user_name: user_name.to_string(),
            tokens: 0,
        }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn user_name(&self) -> &str {
        &self.user_name
    }

    pub fn add_tokens(&mut self, tokens: u64) {
        self.tokens += tokens as i64;
    }

    pub fn subtract_tokens(&mut self, tokens: u64) {
        self.tokens -= tokens as i64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_account_should_have_zero_tokens() {
        let user_name = "test_user";
        let account = Account::new(user_name);

        assert_eq!(account.user_name, user_name);
        assert_eq!(account.tokens, 0);
    }

    #[test]
    fn add_tokens_should_increase_token_count() {
        let mut account = Account::new("test_user");

        account.add_tokens(100);
        assert_eq!(account.tokens, 100);

        account.add_tokens(50);
        assert_eq!(account.tokens, 150);
    }

    #[test]
    fn subtract_tokens_should_decrease_token_count() {
        let mut account = Account::new("test_user");

        account.subtract_tokens(100);
        assert_eq!(account.tokens, -100);

        account.subtract_tokens(50);
        assert_eq!(account.tokens, -150);
    }
}
