use solana_program::pubkey::{Pubkey, PubkeyError};
use borsh::BorshSerialize;
use borsh::BorshDeserialize;
use crate::id;
use crate::{COUNTER_SEED, SETTINGS_SEED};

/// New counter for every user
#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
pub struct Counter {
    /// Value of a counter
    pub value: i64,
}

impl Counter {
    pub fn generate_counter_pk(user: &Pubkey) -> Result<Pubkey, PubkeyError> {
        Pubkey::create_with_seed(user, COUNTER_SEED, &id())
    }

    pub fn check_counter_pk(user: &Pubkey, transaction: &Pubkey) -> bool {
        let counter = Self::generate_counter_pk(user);
        if let Ok(pk) = counter {
            transaction.to_bytes() == pk.to_bytes()
        } else {
            false
        }
    }
}

#[cfg(test)]
mod counter_test {
    use borsh::BorshSerialize;
    use borsh::BorshDeserialize;
    use crate::state::Counter;
    use solana_program::pubkey::Pubkey;
    use std::str::FromStr;

    const COUNTER: Counter = Counter { value: -777 };
    const BINARY_COUNTER: [u8; 8] = [247, 252, 255, 255, 255, 255, 255, 255];

    #[test]
    fn when_serialization_counter_expect_ok() {
        let serialized_counter = COUNTER.try_to_vec().unwrap();

        assert_eq!(serialized_counter, BINARY_COUNTER)
    }

    #[test]
    fn when_deserialization_counter_expect_ok() {
        let deserialized_counter = Counter::try_from_slice(&BINARY_COUNTER).unwrap();

        assert_eq!(deserialized_counter, COUNTER)
    }

    #[test]
    fn when_generate_counter_pk_expect_equals() {
        let user_pk = Pubkey::from_str("4UPHhQxnJrsmLE5w1qLencgCCttYiPswdaRRpQ9xwG5Z").unwrap();
        let generated_pk = Counter::generate_counter_pk(&user_pk).unwrap();

        let counter_pk = Pubkey::from_str("Ffav6rApgVYVogddJrLsccYwveUZCS8KJoM5TLW8T6CH").unwrap();

        assert_eq!(generated_pk, counter_pk)
    }

    #[test]
    fn when_check_counter_pk_expect_transaction_pk_true() {
        let user_pk = Pubkey::from_str("4UPHhQxnJrsmLE5w1qLencgCCttYiPswdaRRpQ9xwG5Z").unwrap();
        let counter_pk = Pubkey::from_str("Ffav6rApgVYVogddJrLsccYwveUZCS8KJoM5TLW8T6CH").unwrap();

        let check = Counter::check_counter_pk(&user_pk, &counter_pk);

        assert_eq!(check, true)
    }

    #[test]
    fn when_check_counter_pk_expect_transaction_pk_false() {
        let user_pk = Pubkey::from_str("4UPHhQxnJrsmLE5w1qLencgCCttYiPswdaRRpQ9xwG5Z").unwrap();
        let wrong_counter_pk = Pubkey::from_str("2wY7hT8TJhFpQqQJ5PGSed76vEgGNeQ11y1PvPsLUcS4").unwrap(); // admin pk

        let check = Counter::check_counter_pk(&user_pk, &wrong_counter_pk);

        assert_eq!(check, false)
    }
}

/// Settings for every counter
#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
pub struct Settings {
    /// Account admin
    pub admin: Pubkey,

    /// Increment step
    pub inc_step: u32,

    /// Decrement step
    pub dec_step: u32,
}

impl Settings {

    pub fn create_signer_seed(bump: &[u8]) -> [&[u8]; 2] {
        [SETTINGS_SEED.as_bytes(), bump]
    }

    pub fn get_settings_pk_with_bump() -> (Pubkey, u8) {
        Pubkey::find_program_address(&[SETTINGS_SEED.as_bytes()], &id())
    }

    pub fn check_settings_pk(settings: &Pubkey) -> bool {
        let (pk, _) = Self::get_settings_pk_with_bump();
        pk.to_bytes() == settings.to_bytes()
    }
}

#[cfg(test)]
mod settings_test {
    use borsh::BorshSerialize;
    use borsh::BorshDeserialize;
    use solana_program::pubkey::Pubkey;
    use crate::state::Settings;
    use std::str::FromStr;

    const PK: Pubkey = Pubkey::new_from_array([3_u8; 32]);
    const SETTINGS: Settings = Settings { admin: PK, inc_step: 1, dec_step: 10 };
    const BINARY_SETTINGS: [u8; 40] = [3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
        3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 1, 0, 0, 0, 10, 0, 0, 0];

    #[test]
    fn when_serialization_settings_expect_ok() {
        let serialized_settings: Vec<u8> = SETTINGS.try_to_vec().unwrap();

        assert_eq!(serialized_settings, BINARY_SETTINGS)
    }

    #[test]
    fn when_deserialization_settings_expect_ok() {
        let deserialized_settings = Settings::try_from_slice(&BINARY_SETTINGS).unwrap();

        assert_eq!(deserialized_settings, SETTINGS)
    }

    #[test]
    fn when_get_settings_pk_expect_key() {
        let (generated_pk, bump) = Settings::get_settings_pk_with_bump();
        let settings_pk = Pubkey::from_str("5KCTQH1ZLtbm3C9AmatRBt4roj6yjoVErS2xMkLAN3nA").unwrap();

        assert_eq!(generated_pk, settings_pk);
        assert_eq!(bump, 255);
    }

    #[test]
    fn when_check_settings_pk_expect_true() {
        let settings_pk = Pubkey::from_str("5KCTQH1ZLtbm3C9AmatRBt4roj6yjoVErS2xMkLAN3nA").unwrap();
        let check = Settings::check_settings_pk(&settings_pk);

        assert_eq!(check, true)
    }

    #[test]
    fn when_check_settings_pk_expect_false() {
        let wrong_settings_pk = Pubkey::from_str("2wY7hT8TJhFpQqQJ5PGSed76vEgGNeQ11y1PvPsLUcS4").unwrap(); // admin pk
        let check = Settings::check_settings_pk(&wrong_settings_pk);

        assert_eq!(check, false)
    }
}