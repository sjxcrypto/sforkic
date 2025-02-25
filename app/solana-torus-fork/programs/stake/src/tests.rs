use super::*;
use solana_program::clock::Epoch;
use solana_program::system_instruction::create_account;
use solana_program_test::*;
use solana_sdk::{
    signature::Signer,
    transaction::Transaction,
};

#[tokio::test]
async fn test_reward_distribution() {
    let program_test = ProgramTest::new(
        "stake",
        stake::id(),
        processor!(stake::process_instruction),
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Test reward calculation
    let holdings = 100_000;
    let reward = calculate_rewards(holdings);
    assert_eq!(reward, 13); // 5% yearly reward, distributed daily

    // Test reward distribution
    let mut transaction = Transaction::new_with_payer(
        &[create_account(
            &payer.pubkey(),
            &payer.pubkey(),
            100_000,
            0,
            &stake::id(),
        )],
        Some(&payer.pubkey()),
    );

    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();
}