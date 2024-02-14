#!/bin/zsh


echo "Deploying contract..."
seid tx wasm store ./target/wasm32-unknown-unknown/release/coin_flip.wasm -y --from=admin --chain-id=sei-chain --node tcp://127.0.0.1:26657 --gas=15000000 --fees=1500000usei --broadcast-mode=block


echo "Deployment complete."


