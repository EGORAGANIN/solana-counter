pub mod error;
pub mod processor;
pub mod instruction;
pub mod state;

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;

pub const COUNTER_SEED: &str = "counter";
pub const SETTINGS_SEED: &str = "settings";

solana_program::declare_id!("7eWFSioVjHdJjbobEZu6hn5QLhmjWSv7qLMyCuzamYCG");