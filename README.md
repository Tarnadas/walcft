# walcft

This repository contains the source for the fungible token contract for the WALC project, which is deployed on Near mainnet at address [walc.near](https://nearblocks.io/address/walc.near).

In order to interact with the contract please install [Near CLI](https://github.com/near/near-cli).

## Deployment

```sh
# set environment to mainnet
export NEAR_ENV=mainnet
export CONTRACT_ID=walc.near

# login with Near wallet
near login

# build contract
./build_docker.sh

# deploy WASM binary to Near
near deploy --wasmFile out/fungible_token.wasm --accountId $CONTRACT_ID

# initialize contract state
near call $CONTRACT_ID new_default_meta '{"owner_id": "'$CONTRACT_ID'", "total_supply": "5000000000000000000000000000000000"}' --accountId $CONTRACT_ID

# check balance for owner
near view $CONTRACT_ID ft_balance_of '{"account_id": "'$CONTRACT_ID'"}'
```

## Migration

The v1 version of the contract, which is the first version deployed on Near mainnet needs a state migration for the new WASM binary to work. The reasoning behind the update can be read in this [Pull Request](https://github.com/walc-labs/walcft/pull/1).

In order to run the migration on Near mainnet the new WASM binary needs to be deployed first. Then the `migrate` function needs to be run, which can only be done by the contract address itself.

```sh
# set environment to mainnet
export NEAR_ENV=mainnet
export CONTRACT_ID=walc.near

# login with Near wallet, if not already done
#near login

# deploy new WASM binary to Near
near deploy --wasmFile out/fungible_token.wasm --accountId $CONTRACT_ID

# migrate state
near call $CONTRACT_ID migrate '' --accountId $CONTRACT_ID

# verify that state migration worked, by calling any function without throwing an error
near view $CONTRACT_ID ft_balance_of '{"account_id": "'$CONTRACT_ID'"}'
```
