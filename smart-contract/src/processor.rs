use solana_program::pubkey::Pubkey;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use borsh::BorshSerialize;
use borsh::BorshDeserialize;
use solana_program::account_info::next_account_info;
use solana_program::program_error::ProgramError;
use solana_program::rent::Rent;
use solana_program::sysvar::Sysvar;
use solana_program::program::invoke_signed;
use solana_program::system_instruction;
use crate::instruction::CounterInstruction;
use crate::state::{Counter, Settings};
use crate::error::CounterError;
use crate::id;

pub struct Processor;

impl Processor {

    pub fn process(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        raw_data: &[u8],
    ) -> ProgramResult {
        msg!("Processor::process: {:?}", raw_data);
        let instruction = CounterInstruction::try_from_slice(raw_data)?;
        match instruction {
            CounterInstruction::Inc => Self::process_operation(accounts, instruction),
            CounterInstruction::Dec => Self::process_operation(accounts, instruction),
            CounterInstruction::Reset => Self::process_reset(accounts),
            CounterInstruction::UpdSett { admin, inc_step, dec_step } =>
                Self::process_upd_sett(accounts, admin, inc_step, dec_step)
        }
    }

    fn process_operation(accounts: &[AccountInfo], inst: CounterInstruction) -> ProgramResult {
        msg!("Processor:process_operation inst={:?}", inst);

        let acc_iter = &mut accounts.iter();
        let user_acc = next_account_info(acc_iter)?;
        let counter_acc = next_account_info(acc_iter)?;
        let settings_acc = next_account_info(acc_iter)?;

        // precondition checks
        if !user_acc.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
        if !counter_acc.is_writable
            && !Counter::check_counter_pk(user_acc.key, counter_acc.key) {
            return Err(CounterError::WrongCounterPDA.into());
        }
        if !Settings::check_settings_pk(settings_acc.key) {
            return Err(CounterError::WrongCounterPDA.into());
        }

        let settings: Settings = Settings::try_from_slice(&settings_acc.data.borrow())?;
        let mut counter: Counter = Counter::try_from_slice(&counter_acc.data.borrow())?;

        match inst {
            CounterInstruction::Inc => counter.value += settings.inc_step as i64,
            CounterInstruction::Dec => counter.value -= settings.dec_step as i64,
            _ => panic!("Processor:process_operation incorrect inst={:?}", inst)
        }

        counter.serialize(&mut &mut counter_acc.data.borrow_mut()[..])?;
        msg!("Processor:process_operation done inst={:?}", inst);
        Ok(())
    }

    fn process_reset(accounts: &[AccountInfo]) -> ProgramResult {
        msg!("Processor:process_reset");

        let acc_iter = &mut accounts.iter();
        let user_acc = next_account_info(acc_iter)?;
        let counter_acc = next_account_info(acc_iter)?;

        // precondition checks
        if !user_acc.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
        if !counter_acc.is_writable
            && !Counter::check_counter_pk(user_acc.key, counter_acc.key) {
            return Err(CounterError::WrongCounterPDA.into());
        }

        let mut counter: Counter = Counter::try_from_slice(&counter_acc.data.borrow())?;

        counter.value = 0;

        counter.serialize(&mut &mut counter_acc.data.borrow_mut()[..])?;
        msg!("Processor:process_reset done");
        Ok(())
    }

    fn process_upd_sett(
        accounts: &[AccountInfo],
        admin: Pubkey,
        inc_step: u32,
        dec_step: u32,
    ) -> ProgramResult {
        msg!("Processor:process_upd_sett");

        let acc_iter = &mut accounts.iter();
        let admin_acc = next_account_info(acc_iter)?;
        let settings_acc = next_account_info(acc_iter)?;
        let rent_acc = next_account_info(acc_iter)?;
        let sys_acc = next_account_info(acc_iter)?;

        if !admin_acc.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
        if !admin_acc.is_writable {
            return Err(CounterError::AdminRequired.into());
        }
        if !Settings::check_settings_pk(settings_acc.key) {
            return Err(ProgramError::InvalidArgument);
        }

        if settings_acc.data_is_empty() {
            Self::create_settings_account(
                admin_acc,
                settings_acc,
                sys_acc,
                rent_acc,
                inc_step,
                dec_step
            )?;
        }

        let mut settings: Settings = Settings::try_from_slice(&settings_acc.data.borrow())?;
        if settings.admin != *admin_acc.key && settings.admin != Pubkey::new(&[0_u8; 32]) {
            return Err(CounterError::AdminRequired.into());
        }

        settings.admin = admin;
        settings.inc_step = inc_step;
        settings.dec_step = dec_step;

        settings.serialize(&mut &mut settings_acc.data.borrow_mut()[..])?;
        msg!("Processor:process_upd_sett done");
        Ok(())
    }

    fn create_settings_account<'a>(
        admin_acc: &AccountInfo<'a>,
        settings_acc: &AccountInfo<'a>,
        sys_acc: &AccountInfo<'a>,
        rent_acc: &AccountInfo<'a>,
        inc_step: u32,
        dec_step: u32,
    ) -> ProgramResult {
        msg!("Creating settings account");
        let settings = Settings { admin: admin_acc.key.clone(), inc_step, dec_step };

        let space = settings.try_to_vec()?.len();
        let rent = Rent::from_account_info(rent_acc)?;
        let rent_value = rent.minimum_balance(space);
        let (settings_pk, bump) = Settings::get_settings_pk_with_bump();
        let bump_ref = &[bump];
        let signer_seeds: &[&[_]] = &Settings::create_signer_seed(bump_ref);

        let create_settings_acc_instr = system_instruction::create_account(
            admin_acc.key,
            &settings_pk,
            rent_value,
            space as u64,
            &id(),
        );

        invoke_signed(
            &create_settings_acc_instr,
            &[admin_acc.clone(), settings_acc.clone(), sys_acc.clone()],
            &[signer_seeds],
        )?;
        msg!("Creating settings account done");
        Ok(())
    }
}
