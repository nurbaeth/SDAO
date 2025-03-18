# 🏛 ThanosDAO - Solana Smart Contract

**ThanosDAO** is a decentralized governance smart contract built on **Solana** using **Anchor**. It allows users to propose ideas, vote on them, and execute proposals if they receive enough votes.

## 🚀 Features
- **Create Proposals** – Only the owner can create new proposals.
- **Vote** – Users can vote once per proposal.
- **Execute** – A proposal can be executed if it gets enough votes.

## 🛠 Installation
 
### 1️⃣ Install Rust & Solana CLI
```sh 
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```
 
### 2️⃣ Install Anchor  
```sh 
cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked
```

### 3️⃣ Clone the Repository
```sh 

```

### 4️⃣ Build the Contract
```sh
anchor build
```

### 5️⃣ Deploy to Localnet
```sh
solana-test-validator
anchor deploy
```

## 📜 Usage

### Create a Proposal
```sh
anchor test -- --nocapture
```

### Vote on a Proposal
Modify the test script to include a voting transaction and run it again.

### Execute a Proposal
Modify the test script to execute the proposal and rerun it.

## 📜 License
This project is licensed under the **MIT License**.

---
💡 **Build the future with ThanosDAO!**
