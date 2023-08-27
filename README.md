# walcft


NOTES
near login

./build

near deploy --wasmFile out/contract.wasm --accountId $WALCTEST2_CONTRACT_ID

near call $WALCTEST2_CONTRACT_ID new_default_meta '{"owner_id": "'$WALCTEST2_CONTRACT_ID'", "total_supply": "5000000000000000000000000000000000"}' --accountId $WALCTEST2_CONTRACT_ID
near call $WALCTEST2_CONTRACT_ID storage_deposit '{"account_id": "'$WALCTEST2_CONTRACT_ID'"}' --accountId $WALCTEST2_CONTRACT_ID --amount 0.01

near view $WALCTEST2_CONTRACT_ID ft_balance_of '{"account_id": "'$WALCTEST2_CONTRACT_ID'"}'


####


export NEAR_ENV=mainnet
near login

export WALCFT_PRODUCTION_ID=walc.near



near deploy --wasmFile out/contract.wasm --accountId $WALCFT_PRODUCTION_ID

near call $WALCFT_PRODUCTION_ID new_default_meta '{"owner_id": "'$WALCFT_PRODUCTION_ID'", "total_supply": "5000000000000000000000000000000000"}' --accountId $WALCFT_PRODUCTION_ID
near call $WALCFT_PRODUCTION_ID storage_deposit '{"account_id": "'$WALCFT_PRODUCTION_ID'"}' --accountId $WALCFT_PRODUCTION_ID --amount 0.01

near view $WALCFT_PRODUCTION_ID ft_balance_of '{"account_id": "'$WALCFT_PRODUCTION_ID'"}'



14044213143421888300000000

