use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    sysvar::clock::Clock,
    sysvar::Sysvar,
    program_error::ProgramError,
    hash::{Hash, Hasher},
};

pub fn calculate_rewards(user_holdings: u64) -> u64 {
    ((user_holdings * 5) / 100) / 365
}

pub fn verify_reward_distribution(
    user_account: &AccountInfo,
    reward_hash: Hash,
) -> bool {
    let mut hasher = solana_program::hash::Hasher::default();
    hasher.hash(&user_account.lamports().to_le_bytes());
    hasher.hash(&reward_hash.to_bytes());

    let expected_hash = Hash::new_unique(); // Replace with actual stored hash
    hasher.result() == expected_hash
}

pub fn distribute_rewards(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let stake_account = next_account_info(accounts_iter)?;
    let user_account = next_account_info(accounts_iter)?;
    let clock = Clock::get()?;

    let user_holdings = stake_account.lamports();
    let reward = calculate_rewards(user_holdings);

    if verify_reward_distribution(user_account, Hash::new_unique()) {
        **stake_account.try_borrow_mut_lamports()? -= reward;
        **user_account.try_borrow_mut_lamports()? += reward;
    }

    Ok(())
}