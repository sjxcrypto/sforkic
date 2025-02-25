# Solana Fork with Torus-Core-Inspired Reward Algorithm

This is a fork of the Solana network with a modified reward algorithm that distributes a 5% yearly reward based on user holdings.

## Setup
1. Clone the repository.
2. Install Rust and dependencies.
3. Run the testnet.

## Reward Algorithm
- Rewards are calculated daily using the formula: `Daily Reward = (User Holdings * 0.05) / 365`.

## Deployment
- Run the deployment script: `./deploy.sh`.