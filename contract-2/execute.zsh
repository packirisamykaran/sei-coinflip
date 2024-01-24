
echo "Executing contract withdrawal"

# seid tx wasm execute sei14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sh9m79m '{"Withdraw":{"amount":9992999999973500000}}' --node tcp://127.0.0.1:26657 --chain-id sei-chain
seid tx wasm execute sei1nc5tatafv6eyq7llkr2gv50ff9e22mnf70qgjlv737ktmt4eswrqms7u8a '{"withdraw":{"amount":"9992999999973500000"}}' --from sei1529r93wfs7j5erfvxqhvzpae35xnse9qz29hz0 --node tcp://127.0.0.1:26657 --chain-id sei-chain --gas 200000 --fees 500000usei -y --broadcast-mode block
echo "Contract execution complete."