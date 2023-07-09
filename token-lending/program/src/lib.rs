#![deny(missing_docs)]

//! A lending program for the Solana blockchain.

pub mod entrypoint;
pub mod processor;
pub use solend_sdk::{error, instruction, math, oracles, state};

// Export current sdk types for downstream users building with a different sdk version
pub use solana_program;

solana_program::declare_id!("58HMHe7X1DnnjfHY5SuMi9ZwekPy2C1dSsnaYH3Ln2z8");

/// Canonical null pubkey. Prints out as "nu11111111111111111111111111111111111111111"
pub const NULL_PUBKEY: solana_program::pubkey::Pubkey =
    solana_program::pubkey::Pubkey::new_from_array([
        11, 193, 238, 216, 208, 116, 241, 195, 55, 212, 76, 22, 75, 202, 40, 216, 76, 206, 27, 169,
        138, 64, 177, 28, 19, 90, 156, 0, 0, 0, 0, 0,
    ]);
