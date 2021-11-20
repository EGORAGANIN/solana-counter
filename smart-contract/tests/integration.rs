#![cfg(feature = "test-bpf")]

use std::borrow::Borrow;
use solana_program::system_instruction;
use solana_program_test::{processor, ProgramTest, ProgramTestContext};
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;
use counter::COUNTER_SEED;
use counter::instruction::CounterInstruction;
use counter::state::{Counter, Settings};
use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use counter::id;
use counter::entrypoint::process_instruction;
use solana_program::pubkey::Pubkey;

struct Env {
    ctx: ProgramTestContext,
    admin: Keypair,
    user: Keypair,
}

impl Env {
    async fn new() -> Self {
        let counter_program = ProgramTest::new("counter", id(), processor!(process_instruction));
        let mut ctx = counter_program.start_with_context().await;

        let admin = Keypair::new();
        let user = Keypair::new();


        // Deposit SOL
        let admin_deposit_instr = system_instruction::transfer(
            &ctx.payer.pubkey(),
            &admin.pubkey(),
            5_000_000_000,
        );
        let user_deposit_instr = system_instruction::transfer(
            &ctx.payer.pubkey(),
            &user.pubkey(),
            3_000_000_000,
        );
        let deposit_tx = Transaction::new_signed_with_payer(
            &[admin_deposit_instr, user_deposit_instr],
            Some(&ctx.payer.pubkey()),
            &[&ctx.payer],
            ctx.last_blockhash,
        );
        ctx.banks_client.process_transaction(deposit_tx).await.unwrap();


        // Update settings
        let upd_sett_instr = CounterInstruction::upd_sett_instr(
            admin.pubkey(),
            admin.pubkey(),
            9,
            5,
        );
        let update_settings_tx = Transaction::new_signed_with_payer(
            &[upd_sett_instr],
            Some(&admin.pubkey()),
            &[&admin],
            ctx.last_blockhash,
        );
        ctx.banks_client.process_transaction(update_settings_tx).await.unwrap();

        // Check settings account
        let settings_pk = Settings::get_settings_pk_with_bump().0;
        let settings_acc = ctx.banks_client.get_account(settings_pk).await.unwrap().unwrap();
        let deserialized_settings = Settings::try_from_slice(settings_acc.data.borrow()).unwrap();
        let inited_settings = Settings { admin: admin.pubkey(), inc_step: 9, dec_step: 5 };
        assert_eq!(deserialized_settings, inited_settings);


        // Init counter account
        let counter = Counter { value: 0 };
        let space = counter.try_to_vec().unwrap().len();
        let rent = ctx.banks_client.get_rent().await.unwrap();
        let rent_value = rent.minimum_balance(space);

        let create_counter_instr = system_instruction::create_account_with_seed(
            &user.pubkey(),
            &Counter::generate_counter_pk(&user.pubkey()).unwrap(),
            &user.pubkey(),
            COUNTER_SEED,
            rent_value,
            space as u64,
            &id(),
        );
        let create_counter_tx = Transaction::new_signed_with_payer(
            &[create_counter_instr],
            Some(&user.pubkey()),
            &[&user],
            ctx.last_blockhash,
        );
        ctx.banks_client.process_transaction(create_counter_tx).await.unwrap();


        // Check counter account
        let counter_pk = Counter::generate_counter_pk(&user.pubkey()).unwrap();
        let counter_acc = ctx.banks_client.get_account(counter_pk).await.unwrap().unwrap();
        let deserialized_counter = Counter::try_from_slice(&counter_acc.data.borrow()).unwrap();
        assert_eq!(deserialized_counter, counter);

        Env { ctx, admin, user }
    }
}

#[tokio::test]
async fn inc() {
    let env = Env::new().await;
    let user = env.user;
    let mut ctx = env.ctx;

    let inc_instr = CounterInstruction::inc_instr(user.pubkey());
    let inc_tx = Transaction::new_signed_with_payer(
        &[inc_instr],
        Some(&user.pubkey()),
        &[&user],
        ctx.last_blockhash,
    );
    ctx.banks_client.process_transaction(inc_tx).await.unwrap();

    let counter_acc = ctx
        .banks_client
        .get_account(Counter::generate_counter_pk(&user.pubkey()).unwrap())
        .await
        .unwrap()
        .unwrap();
    let counter = Counter::try_from_slice(&counter_acc.data.borrow()).unwrap();

    assert_eq!(counter.value, 9);
}

#[tokio::test]
async fn dec() {
    let env = Env::new().await;
    let user = env.user;
    let mut ctx = env.ctx;

    let dec_instr = CounterInstruction::dec_instr(user.pubkey());
    let dec_tx = Transaction::new_signed_with_payer(
        &[dec_instr],
        Some(&user.pubkey()),
        &[&user],
        ctx.last_blockhash,
    );
    ctx.banks_client.process_transaction(dec_tx).await.unwrap();

    let counter_acc = ctx
        .banks_client
        .get_account(Counter::generate_counter_pk(&user.pubkey()).unwrap())
        .await
        .unwrap()
        .unwrap();
    let counter = Counter::try_from_slice(&counter_acc.data.borrow()).unwrap();

    assert_eq!(counter.value, -5);
}

#[tokio::test]
async fn reset() {
    let env = Env::new().await;
    let user = env.user;
    let mut ctx = env.ctx;

    let reset_instr = CounterInstruction::reset_instr(user.pubkey());
    let reset_tx = Transaction::new_signed_with_payer(
        &[reset_instr],
        Some(&user.pubkey()),
        &[&user],
        ctx.last_blockhash,
    );
    ctx.banks_client.process_transaction(reset_tx).await.unwrap();

    let counter_acc = ctx
        .banks_client
        .get_account(Counter::generate_counter_pk(&user.pubkey()).unwrap())
        .await
        .unwrap()
        .unwrap();
    let counter = Counter::try_from_slice(&counter_acc.data.borrow()).unwrap();

    assert_eq!(counter.value, 0);
}

#[tokio::test]
async fn upd_sett() {
    let env = Env::new().await;
    let admin = env.admin;
    let mut ctx = env.ctx;

    let upd_sett_instr = CounterInstruction::upd_sett_instr(admin.pubkey(), admin.pubkey(), 1, 2);
    let upd_sett_tx = Transaction::new_signed_with_payer(
        &[upd_sett_instr],
        Some(&admin.pubkey()),
        &[&admin],
        ctx.last_blockhash,
    );
    ctx.banks_client.process_transaction(upd_sett_tx).await.unwrap();

    let settings_acc = ctx
        .banks_client
        .get_account(Settings::get_settings_pk_with_bump().0)
        .await
        .unwrap()
        .unwrap();
    let settings: Settings = Settings::try_from_slice(&settings_acc.data.borrow()).unwrap();

    assert_eq!(settings.inc_step, 1);
    assert_eq!(settings.dec_step, 2);
}
