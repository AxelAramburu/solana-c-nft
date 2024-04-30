## Minting cNFT on Solana Blockchain 

That a project with tests example on how to mint a cNFT as part of a collection on Solana blockchain.

It's a project example with program/accounts deployed on the Devnet

Steps to reproduce: 
1) Configure you're Solana Dev ecosystem, keypair, etc.
2) Add the variables on the `.env` file.
3) Install the dependencies with `npm i`.
4) Build the solana program (writed in Rust) with `anchor build`.
5) Deploy the program with: `anchor deploy`.
6) Now take the deployed programID, and replace the `declare_id!(******)` field on the `programs/solana-c-nft/src/lib.rs` and the program address on `Anchor.toml` file by the program deployed.
7) Re-run anchor build.
8) Run `anchor test` to execute.
  
Big shoutout to Discord Metaplex and Anchor community for good ressources 🤝

### Logs txs example :

```
Program:  3raZxd2zrYgu3qQ8mwpKBK3wxoUBedHm6Lt5GPpMzSaB
Signer:  4ZNVvrgCqybh44j7iT8FsTzB7yHeKmyBJdxtrUk5JeDv
Collection PDA:  5vkQ69edm1denZvFpLotkBFHZoNsFusi4qidBvN1tJut
ATA Pubkey:  8XVxZ1npfRQg5EHqD2o61pYDZXHBbENxcCBEkwjrGxeM
Metadata account:  2Zf25hbseFtCTWJnPTX1NE9ZijGKsznYJf5SehB9Sv6h
Master Edition account:  AfU53WjasqLy8ZLHRMgZM1NX7tXvrJJ7fNY11eMWfmfC
Merkle Tree:  6YBPFQ4T2oZDBRGRhRAnDmLAiXw5nMA2yCacQQqZGFf6
Merkle Tree Auth:  5scxHp6GtcN2kiWeYxXKJQ7LXEGvFZTTzuH8Ndszydxt


Init CNft Tx:  v1RWmSYMbGRavg44wNBtuyixkBGNFcGDjfrtGeiH1uhu4SbipQe8HdXj8tVZgwfNsmYT3N2UcC1ydJiCqvRaJnf
  ✔ Initialize cNFT! (1310ms)

Max Depth computed:  14
Creation of the tree tx:  FAanwdcQSFYTC42TiU89MC684VWfQfXtjHhL2ptGa5D5keDj1Lj5Ywp52fFDA156crwVovqa1erNtA2VhWu5kky
Tree Tx:  3z7e5RpJsCn3nH12XrdReRUJViufqmg4zYosDBnpsaBzKam6qKayTj8wN7aQZVhn8cc8B1L9TaKWPGpXjKT5Y9mS
  ✔ Initialize Tree (4924ms)
Mint tx:  5AwJVTRpJCxuKd8Lr4JPW4Rz7ABsCsoMXsiU4Et6CKj4YkvuPSxYNb8KxvVJHxAamSM3hwDymnMErLh9dec3ZtCP
  ✔ Mint a cNft token ! (3203ms)

  2 passing (8s)
```
