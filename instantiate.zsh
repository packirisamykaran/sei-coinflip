echo "Instantiating contract..."
seid tx wasm instantiate 2 '{}' --label "my-test-contract" --admin sei1529r93wfs7j5erfvxqhvzpae35xnse9qz29hz0 --from admin --chain-id sei-chain --node tcp://127.0.0.1:26657 --amount 9992999999973500000usei --gas 200000 --fees 500000usei -y --broadcast-mode block



echo "Instantiation complete."

