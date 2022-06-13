## Build and Init

```bash
export ACCOUNT_ID=edricngo.testnet
export ACCOUNT_ID2=cert.edricngo.testnet
./build.sh
near deploy --wasmFile target/wasm32-unknown-unknown/release/certy_cert.wasm --accountId $CONTRACT_ID
near call $CONTRACT_ID new_default_meta '{"owner_id": "edricngo.testnet"}' --accountId edricngo.testnet
```

## Create category

```bash
near call $CONTRACT_ID category_create '{ "category_id": "uuid_here" ,"metadata":{  "title": "Certy",  "description": "Certy",  "media": "Certy",  "issued_at": 1653258436,  "updated_at":1653258436 ,  "fields": "Certy",  "reference": "Certy"}}' --accountId $ACCOUNT_ID --depositYocto 6150000000000000000000
```

## View category info

```bash
near view $CONTRACT_ID category_info '{"category_id":"0"}'
```

## View categories by owner

```bash
near view $CONTRACT_ID categories_for_owner '{"account_id":"'$ACCOUNT_ID'"}'
```

# Update category

```bash
near call $CONTRACT_ID category_update '{"metadata":{  "title": "Certy updated",  "description": "Certy",  "media": "Certy",  "issued_at": 1653258436,  "updated_at":1653258436 ,  "fields": "Certy",  "reference": "Certy"}, "category_id" : "0"}' --accountId $ACCOUNT_ID --depositYocto 6150000000000000000000
```

# Mint cert

```bash
near call $CONTRACT_ID nft_mint '{"metadata":{  "title": "Certy",  "description": "Certy",  "media": "Certy",  "issued_at": 1653258436,  "updated_at":1653258436 ,  "fields": "Certy",  "reference": "Certy"}, "receiver_id" : "'$ACCOUNT_ID'", "category_id" : "0" }' --accountId $ACCOUNT_ID --depositYocto 9180000000000000000000
```

# View cert by owner

```bash
near view $CONTRACT_ID nft_tokens_for_owner '{"account_id":"'$ACCOUNT_ID'"}'
```

# View cert by category

```bash
near view $CONTRACT_ID cert_get_by_category '{"category_id":"0"}'
```
