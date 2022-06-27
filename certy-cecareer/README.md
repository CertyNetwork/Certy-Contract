# Certy-Contract

## Build and Init

```bash
export ACCOUNT_ID=certynetwork.testnet
export ACCOUNT_ID2=cecareer.certynetwork.testnet
./build.sh
near deploy --wasmFile target/wasm32-unknown-unknown/release/certy_cecareer.wasm --accountId $CONTRACT_ID
near call $CONTRACT_ID new '{"owner_id": "certynetwork.testnet"}' --accountId certynetwork.testnet
```

## Create Job

```bash
near call $CONTRACT_ID job_create '{"job_id": "JOBID1", "metadata" : {"reference": "bafkreiaqvjyh6lgajankws6fg5ximrrl5wpbrarku3ib2o4whgumhv66im", "reference_hash" : "asdnjkasdnksandkanskdnksadnksa"}}' --accountId $ACCOUNT_ID --depositYocto 6150000000000000000000
```

## Update job

```bash
near call $CONTRACT_ID job_update '{"metadata":{ "referencce" : "bafkreiaqvjyh6lgajankws6fg5ximrrl5wpbrarku3ib2o4whgumhv66im", "reference_hash" : "asdnjkasdnksandkanskdnksadnksa" }, "job_id" : "0"}' --accountId $ACCOUNT_ID --depositYocto 6150000000000000000000
```

## View Job info

```bash
near view $CONTRACT_ID job_info '{"job_id":"JOBID1"}'
```

## View Job of owner

```bash
near view $CONTRACT_ID jobs_for_owner '{"account_id":"'$ACCOUNT_ID'"}'
```
