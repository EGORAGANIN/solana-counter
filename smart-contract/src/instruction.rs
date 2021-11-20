use borsh::BorshSerialize;
use borsh::BorshDeserialize;
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::pubkey::Pubkey;
use solana_program::{system_program, sysvar};
use crate::state::{Counter, Settings};
use crate::id;

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub enum CounterInstruction {
    /// Increment counter
    /// 0. [signer] - owner counter
    /// 1. [writable] - counter_account, PDA
    /// 2. [] - settings account, PDA
    Inc,

    /// Decrement counter
    /// 0. [signer] - owner account
    /// 1. [writable] - counter account, PDA
    /// 2. [] - settings account, PDA
    Dec,

    /// Reset counter
    /// 0. [signer] - owner account
    /// 1. [writable] - counter account, PDA
    Reset,

    /// Update counter settings
    /// 0. [signer, writable] - admin account
    /// 1. [writable] - settings account
    /// 2. [] - Rent sysvar (calculate rent for creating settings accounts)
    /// 3. [] - System program (creating accounts, transfer lamports)
    UpdSett { admin: Pubkey, inc_step: u32, dec_step: u32 },
}

impl CounterInstruction {
    pub fn inc_instr(user: Pubkey) -> Instruction {
        Self::operation_instr(user, &CounterInstruction::Inc)
    }

    pub fn dec_instr(user: Pubkey) -> Instruction {
        Self::operation_instr(user, &CounterInstruction::Dec)
    }

    fn operation_instr(user: Pubkey, instr: &CounterInstruction) -> Instruction {
        let counter_pk = Counter::generate_counter_pk(&user).unwrap();
        let (settings_pk, _) = Settings::get_settings_pk_with_bump();
        Instruction::new_with_borsh(
            id(),
            &instr,
            vec![
                AccountMeta::new_readonly(user, true),
                AccountMeta::new(counter_pk, false),
                AccountMeta::new_readonly(settings_pk, false),
            ],
        )
    }

    pub fn reset_instr(user: Pubkey) -> Instruction {
        let counter_pk = Counter::generate_counter_pk(&user).unwrap();
        Instruction::new_with_borsh(
            id(),
            &CounterInstruction::Reset,
            vec![
                AccountMeta::new_readonly(user, true),
                AccountMeta::new(counter_pk, false),
            ],
        )
    }

    pub fn upd_sett_instr(
        current_admin: Pubkey,
        new_admin: Pubkey,
        inc_step: u32,
        dec_step: u32,
    ) -> Instruction {
        let (settings_pk, _) = Settings::get_settings_pk_with_bump();
        Instruction::new_with_borsh(
            id(),
            &CounterInstruction::UpdSett { admin: new_admin, inc_step, dec_step },
            vec![
                AccountMeta::new(current_admin, true),
                AccountMeta::new(settings_pk, false),
                AccountMeta::new_readonly(sysvar::rent::id(), false),
                AccountMeta::new_readonly(system_program::id(), false),
            ],
        )
    }
}

#[cfg(test)]
mod counter_instruction_test {
    use borsh::BorshSerialize;
    use borsh::BorshDeserialize;
    use crate::instruction::CounterInstruction;
    use solana_program::pubkey::Pubkey;
    use std::str::FromStr;

    #[test]
    fn when_serialization_inc_expect_ok() {
        let inc_instr = CounterInstruction::Inc;
        let binary_instr = [0];

        assert_eq!(inc_instr.try_to_vec().unwrap(), binary_instr)
    }

    #[test]
    fn when_deserialization_inc_expect_ok() {
        let binary_instr = [0];
        let instr = CounterInstruction::try_from_slice(&binary_instr).unwrap();

        assert_eq!(instr, CounterInstruction::Inc)
    }

    #[test]
    fn when_serialization_dec_expect_ok() {
        let dec_instr = CounterInstruction::Dec;
        let binary_instr = [1];

        assert_eq!(dec_instr.try_to_vec().unwrap(), binary_instr)
    }

    #[test]
    fn when_deserialization_dec_expect_ok() {
        let binary_instr = [1];
        let instr = CounterInstruction::try_from_slice(&binary_instr).unwrap();

        assert_eq!(instr, CounterInstruction::Dec)
    }

    #[test]
    fn when_serialization_reset_expect_ok() {
        let reset_instr = CounterInstruction::Reset;
        let binary_instr = [2];

        assert_eq!(reset_instr.try_to_vec().unwrap(), binary_instr)
    }

    #[test]
    fn when_deserialization_reset_expect_ok() {
        let binary_instr = [2];
        let instr = CounterInstruction::try_from_slice(&binary_instr).unwrap();

        assert_eq!(instr, CounterInstruction::Reset)
    }

    #[test]
    fn when_serialization_upd_sett_expect_ok() {
        let admin_pk = Pubkey::from_str("2wY7hT8TJhFpQqQJ5PGSed76vEgGNeQ11y1PvPsLUcS4").unwrap();
        let upd_instr = CounterInstruction::UpdSett { admin: admin_pk, inc_step: 2, dec_step: 10 };
        let binary_instr = [3, 28, 212, 59, 165, 120, 246, 217, 222, 54, 146, 40, 15, 29,
            116, 181, 170, 127, 95, 104, 96, 111, 182, 220, 59, 176, 28, 79, 38, 63, 193, 241, 65,
            2, 0, 0, 0, 10, 0, 0, 0];

        assert_eq!(upd_instr.try_to_vec().unwrap(), binary_instr)
    }

    #[test]
    fn when_deserialization_upd_sett_expect_ok() {
        let binary_instr = [3, 28, 212, 59, 165, 120, 246, 217, 222, 54, 146, 40, 15, 29,
            116, 181, 170, 127, 95, 104, 96, 111, 182, 220, 59, 176, 28, 79, 38, 63, 193, 241, 65,
            2, 0, 0, 0, 10, 0, 0, 0];
        let instr = CounterInstruction::try_from_slice(&binary_instr).unwrap();

        let admin_pk = Pubkey::from_str("2wY7hT8TJhFpQqQJ5PGSed76vEgGNeQ11y1PvPsLUcS4").unwrap();
        let upd_instr = CounterInstruction::UpdSett { admin: admin_pk, inc_step: 2, dec_step: 10 };

        assert_eq!(upd_instr, instr)
    }
}