#!/bin/zsh


echo "Deploying contract..."
# seid tx wasm store ./target/wasm32-unknown-unknown/release/contract_2.wasm -y --from=admin --chain-id=sei-chain --node tcp://127.0.0.1:26657 --gas=15000000 --fees=1500000usei --broadcast-mode=block --from=admin --chain-id=sei-chain --node tcp://127.0.0.1:26657
seid tx wasm store ./target/wasm32-unknown-unknown/release/contract_2.wasm -y --broadcast-mode=block --fees=1100000usei --gas=11000000 --from=admin -o json >> deploy.json

echo "Deployment complete."


