use std::error::Error;
use std::borrow::Borrow;
use std::env;
use std::time::Duration;
use counter::state::{Counter, Settings};
use solana_program::pubkey::Pubkey;
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::{Keypair, read_keypair_file};
use counter;
use counter::instruction::CounterInstruction;
use solana_sdk::account::Account;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;
use borsh::BorshSerialize;
use borsh::BorshDeserialize;
use counter::COUNTER_SEED;
use solana_program::system_instruction;

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::new()?;

    app.update_settings()?;
    app.create_counter_account()?;

    app.increment_counter()?;
    app.decrement_counter()?;
    app.reset_counter()?;

    Ok(())
}

fn get_account(client: &RpcClient, pk: &Pubkey) -> Option<Account> {
    let rpc_result = client.get_account_with_commitment(
        pk,
        client.commitment(),
    );
    match rpc_result {
        Ok(resp) => resp.value,
        Err(_) => None
    }
}

struct App {
    rpc: RpcClient,
    user: Keypair,
    admin: Keypair,
    program: Keypair,
    counter_pk: Pubkey,
    settings_pk: Pubkey,
}

impl App {
    fn new() -> Result<App, Box<dyn Error>> {
        // Load keypairs
        println!("Load keypairs");
        let user = read_keypair_file("../keypair/user.json")?;
        let admin = read_keypair_file("../keypair/admin.json")?;
        let program = read_keypair_file("../keypair/program.json")?;
        let counter_pk = Counter::generate_counter_pk(&user.pubkey())?;
        let (settings_pk, _) = Settings::get_settings_pk_with_bump();
        println!("user pk '{:?}'", user.pubkey());
        println!("admin pk '{:?}'", admin.pubkey());
        println!("program pk '{:?}'", program.pubkey());
        println!("counter pk '{:?}'", counter_pk);
        println!("settings pk '{:?}'", settings_pk);

        // Init RPC client
        let args: Vec<String> = env::args().collect();
        let url = args.get(1).unwrap_or(&"http://localhost:8899".to_string()).to_string();
        println!("Init RPC client URL '{}'", url);
        let rpc_client = RpcClient::new_with_timeout_and_commitment(
            url,
            Duration::from_millis(60_000),
            CommitmentConfig::finalized(),
        );
        println!("Init RPC client done");

        Ok(App { rpc: rpc_client, user, admin, program, counter_pk, settings_pk })
    }

    fn update_settings(&self) -> Result<(), Box<dyn Error>> {
        println!("Update counter settings");
        let (recent_hash, _) = self.rpc.get_recent_blockhash()?;
        let upd_sett_instr = CounterInstruction::upd_sett_instr(
            self.admin.pubkey(),
            self.admin.pubkey(),
            2,
            1,
        );
        let upd_sett_tx = Transaction::new_signed_with_payer(
            &[upd_sett_instr],
            Some(&self.admin.pubkey()),
            &[&self.admin],
            recent_hash,
        );
        self.rpc.send_and_confirm_transaction(&upd_sett_tx)?;
        println!("Update counter settings done");
        let settings_acc = self.rpc.get_account(&self.settings_pk)?;
        let settings = Settings::try_from_slice(&settings_acc.data.borrow())?;
        println!("settings '{:?}'", settings);

        Ok(())
    }

    fn create_counter_account(&self) -> Result<(), Box<dyn Error>> {
        let counter_acc = get_account(&self.rpc, &self.counter_pk);
        if counter_acc == None {
            println!("Create counter account");
            let (recent_hash, _) = self.rpc.get_recent_blockhash()?;

            let counter = Counter { value: 0 };
            let space = counter.try_to_vec()?.len();
            let rent_value = self.rpc.get_minimum_balance_for_rent_exemption(space)?;
            let create_counter_acc_instr = system_instruction::create_account_with_seed(
                &self.user.pubkey(),
                &self.counter_pk,
                &self.user.pubkey(),
                COUNTER_SEED,
                rent_value,
                space as u64,
                &self.program.pubkey(),
            );
            let create_counter_acc_tx = Transaction::new_signed_with_payer(
                &[create_counter_acc_instr],
                Some(&self.user.pubkey()),
                &[&self.user],
                recent_hash,
            );
            self.rpc.send_and_confirm_transaction(&create_counter_acc_tx)?;
            println!("Create counter account done");
        }
        let counter_acc = self.rpc.get_account(&self.counter_pk)?;
        let counter = Counter::try_from_slice(&counter_acc.data.borrow())?;
        println!("counter '{:?}'", counter);

        Ok(())
    }

    fn increment_counter(&self) -> Result<(), Box<dyn Error>> {
        println!("Increment counter");
        let (recent_hash, _) = self.rpc.get_recent_blockhash()?;
        let inc_instr = CounterInstruction::inc_instr(self.user.pubkey());
        let inc_tx = Transaction::new_signed_with_payer(
            &[inc_instr],
            Some(&self.user.pubkey()),
            &[&self.user],
            recent_hash,
        );
        self.rpc.send_and_confirm_transaction(&inc_tx)?;
        println!("Increment counter done");

        let counter_acc = self.rpc.get_account(&self.counter_pk)?;
        let counter = Counter::try_from_slice(&counter_acc.data.borrow())?;
        println!("counter '{:?}'", counter);

        Ok(())
    }

    fn decrement_counter(&self) -> Result<(), Box<dyn Error>> {
        println!("Decrement counter");
        let (recent_hash, _) = self.rpc.get_recent_blockhash()?;
        let dec_instr = CounterInstruction::dec_instr(self.user.pubkey());
        let dec_tx = Transaction::new_signed_with_payer(
            &[dec_instr],
            Some(&self.user.pubkey()),
            &[&self.user],
            recent_hash,
        );
        self.rpc.send_and_confirm_transaction(&dec_tx)?;
        println!("Decrement counter done");

        let counter_acc = self.rpc.get_account(&self.counter_pk)?;
        let counter = Counter::try_from_slice(&counter_acc.data.borrow())?;
        println!("counter '{:?}'", counter);

        Ok(())
    }

    fn reset_counter(&self) -> Result<(), Box<dyn Error>> {
        println!("Reset counter");
        let (recent_hash, _) = self.rpc.get_recent_blockhash()?;
        let reset_instr = CounterInstruction::reset_instr(self.user.pubkey());
        let reset_tx = Transaction::new_signed_with_payer(
            &[reset_instr],
            Some(&self.user.pubkey()),
            &[&self.user],
            recent_hash,
        );
        self.rpc.send_and_confirm_transaction(&reset_tx)?;
        println!("Reset counter done");

        let counter_acc = self.rpc.get_account(&self.counter_pk)?;
        let counter = Counter::try_from_slice(&counter_acc.data.borrow())?;
        println!("counter '{:?}'", counter);

        Ok(())
    }
}