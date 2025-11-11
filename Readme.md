# Simple Character Game example

Player moves around and shows in real-time how the mojo sdk manages state seamlessly on-chain

## Running the project [devnet]

_🚨🚨 it is expected that you've already set up your Solana cli and are on Devnet_

```shell
 # create a new keypair
solana-keygen new -s -o dev_wallet-keypair.json
 # airdrop some SOL to your account
solana airdrop 1 $(solana address -k dev_wallet-keypair.json)
 # build and run the project
cargo run
```

when the game window loads up,\
use arrow keys to move the player aroud the screen \
⬆️ ⬇️ ⬅️ ➡️
