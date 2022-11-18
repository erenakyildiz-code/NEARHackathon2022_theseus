# Theseus launchpad integration Contract


# Quickstart

1. Make sure you have installed [rust](https://rust.org/).
2. Install the [`NEAR CLI`](https://github.com/near/near-cli#setup)


## 1. Build and Deploy the Contract
---
You can automatically compile and deploy the contract in the NEAR testnet by running:

```bash
./deploy.sh
```

Once finished, check the `neardev/dev-account` file to find the address in which the contract was deployed:

```bash
cat ./neardev/dev-account
# e.g. dev-1659899566943-21539992274727
```
## IMPORTANT
- You need to take the dev account generated from deployment , put it in frontend's /src/index.js as the CONTRACT_NAME.
- If you want to be admin, change the erentester.testnet accounts in lib.rs to yourtestname.testnet


## 2. Deposit balances (important)
---
From the frontend code, you can deposit and withdraw balances with the click of a button.

- If you skip this step you will not be able to buy any tokens when launch starts.
 

## 3. Stake all balances and start a launch period
---
`stake(poolname)` stakes all NEAR currently deposited to the launchpad on to a chosen pool, if you are a node, you may choose your own pool's name. You need NEAR-cli to be able to run this command.
```bash
# Use near-cli to login your NEAR account
near login
```
```bash
#stake(pool)
near call <SmartContractAccountID > stake '{"pool": "<YourPoolAccountID>"}' --accountId <YourAccountId>
```

`start_launch(account_of_token, amount_to_sell)` Starts a new launch from the platform. before doing so you need to register the contract for the fungible token that you are launching, and send the contract the amount_to_sell amount of tokens beforehand.

```bash

# Use near-cli to mint some tokens and start a launch on our launchpad
near call <fungibleTokenContract> storage_deposit '{"account_id": "<SmartContractAccountId>"}' --accountId <YourAccountId> --amount 0.00125

near call <fungibleTokenContract> ft_mint '{"receiver_id": "<SmartContractAccountId>", "amount": "<amountToSell>"}' --deposit 0.1 --accountId <YourAccountId>

near call <SmartContractAccountId> start_launch '{"account_of_token":"<account of a fungible token contract>", "amount_to_sell":"<amountToSell>" }' --accountId <YourAccountId>
```

**Tip:** use ft.examples.testnet as your first token, it is easier.
## 4. Check the frontend to start purchasing the tokens

Congrats, you have successfully started a token launch, hope you deposited some money beforehand, or you will have to do everything from scratch.
You can now press purchase allocations button from the frontend to purchase all tokens that are allocated to you, currently every token is 1 satoshi(in NEAR). This can be changed later with the usage of oracles etc.

Begin license text.
Copyright 2022 eren akyıldız

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

End license text.