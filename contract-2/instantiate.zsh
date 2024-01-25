echo "Instantiating contract..."
seid tx wasm instantiate 41 '{}' --label "contract2" --admin sei1529r93wfs7j5erfvxqhvzpae35xnse9qz29hz0 --from admin --chain-id sei-chain --node tcp://127.0.0.1:26657 --amount 100000000usei --gas 1000000 --fees 100000usei -y --broadcast-mode block -o json >> instantiate.json



echo "Instantiation complete."

