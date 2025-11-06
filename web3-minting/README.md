# Voice-First Web3 Minting Service

A Rust-based backend service that handles metadata preparation and NFT minting for the voice-first Web3 concierge application.

🚀 Overview

This service is the minting module of the full architecture. After a user interacts via voice and receives a response from the agent system, this module takes over to package the result (e.g., transcript, audio link) into metadata, and mints an NFT (or token) on a blockchain or via a 3rd-party minting API.

🧩 Key Features

Accepts mint requests with wallet address and metadata payload

Uploads asset metadata (image/audio/text) to storage (e.g., IPFS)

Calls blockchain smart contract or external minting API to mint the asset

Returns token ID, transaction hash, and asset link to the frontend

Provides optional endpoints to query minting status or list assets

🛠 Tech Stack

Rust – core backend logic

Web framework: e.g., actix-web or rocket

HTTP client: reqwest (for calling storage & minting APIs)

Blockchain library: e.g., ethers-rs (if minting directly)

Storage interface: IPFS, Arweave or equivalent

Environment config: .env for RPC URL, private keys, contract address, etc

📁 Repository Structure
web3-minting/
├── Cargo.toml
├── .env.example
├── README.md
├── src/
│   ├── main.rs
│   ├── config.rs
│   ├── handlers/
│   │   ├── mint.rs
│   │   ├── status.rs
│   │   └── assets.rs
│   ├── services/
│   │   ├── blockchain.rs
│   │   ├── metadata.rs
│   │   └── storage.rs
│   ├── models/
│   │   ├── request.rs
│   │   ├── response.rs
│   │   └── asset.rs
│   ├── error.rs
│   └── util.rs
└── migrations/

🧭 How to Run Locally

Copy .env.example to .env and fill in needed variables (RPC_URL, WALLET_PRIVATE_KEY, CONTRACT_ADDRESS, etc)

Run:

cargo build
cargo run


The service listens on http://localhost:8081 (by default) and exposes endpoints like /mint.


📝 License

MIT – See LICENSE for details.

✨ Acknowledgements

Inspired by the architecture of the voice-first Web3 concierge system that incorporates multi-agent orchestration with voice UI and Web3 integration.