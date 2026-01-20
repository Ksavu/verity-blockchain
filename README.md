![Verity Network](verity-logo.png)

# Verity Network

**The Fastest Privacy-Focused Blockchain**

Verity Network is a high-performance privacy blockchain featuring 5-second block times, complete transaction shielding via zk-SNARKs, and a built-in decentralized exchange.

## 🚀 Features

- ⚡ **5-second blocks** - 24x faster than Monero (120s blocks)
- 🔒 **Complete privacy** - Sender, receiver, and amount all hidden via zk-SNARKs
- 💰 **Ultra-low fees** - ~$0.001 per transaction
- 🔄 **Built-in DEX** - Integrated decentralized exchange for private token swaps
- 🌐 **VTY token** - Native cryptocurrency for the Verity Network

## 📊 Technical Specifications

| Specification | Value |
|--------------|-------|
| Block Time | 5 seconds |
| Privacy | Full zk-SNARK shielding |
| Consensus | CometBFT (Tendermint) |
| Token | VTY (uvty base unit) |
| Address Format | verity1... |
| Validator Prefix | verityvalid1... |
| DEX | Integrated AMM with multi-hop routing |

## 🌐 Ecosystem

- **Block Explorer**: [explorer.veritynetwork.io](https://explorer.veritynetwork.io)
- **DEX Interface**: [Verity Swap](https://verityswap.netlify.app)
- **Mobile Wallet**: [Verity Wallet](https://verity-wallet-pwa.vercel.app)
- **Website**: [veritynetwork.io](https://veritynetwork.io)

## 🛠️ Building from Source

### Prerequisites
- Rust 1.75+
- Go 1.21+ (for CometBFT)
- 8GB RAM minimum

### Build
```bash
# Clone the repository
git clone https://github.com/Ksavu/verity-blockchain.git
cd verity-blockchain

# Build binaries
cargo build --release --bin pd
cargo build --release --bin pcli

# Binaries will be in target/release/
```

## 🚀 Running a Node

### Initialize Testnet
```bash
# Generate testnet config
./target/release/pd network generate

# Start the node
./target/release/pd start --home ~/.penumbra/network_data/node0
```

### Start CometBFT
```bash
# In another terminal
cometbft start --home ~/.penumbra/network_data/node0/cometbft
```

## 💻 Using the CLI Wallet
```bash
# Initialize wallet
./target/release/pcli init --grpc-url http://localhost:8080

# Check balance
./target/release/pcli view balance

# Send VTY
./target/release/pcli tx send 100vty --to <verity-address>
```

## 🔒 Privacy Features

Verity Network provides complete transaction privacy through:

- **Shielded Transfers**: All sender, receiver, and amount data encrypted
- **zk-SNARK Proofs**: Zero-knowledge cryptography ensures validity without revealing data
- **Private DEX Swaps**: Token exchanges with no public order books
- **Confidential Assets**: Create and trade custom tokens privately

## 🔄 Decentralized Exchange

Built-in AMM with:
- Multi-hop routing (up to 4 hops)
- Private liquidity provision
- Shielded order execution
- No front-running possible

## 🌟 Why Verity?

| Feature | Verity | Monero | Penumbra |
|---------|--------|--------|----------|
| Block Time | 5s | 120s | ~5s |
| Privacy | Full | Full | Full |
| Built-in DEX | ✅ | ❌ | ✅ |
| Speed Advantage | 24x vs Monero | - | Similar |

## 📖 Documentation

- [Technical Whitepaper](docs/whitepaper.md)
- [API Documentation](docs/api.md)
- [Validator Guide](docs/validators.md)

## 🤝 Contributing

Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md) first.

## 📄 License

Licensed under Apache 2.0 and MIT

## 🔗 Links

- [Discord](https://discord.gg/verity)
- [Twitter](https://twitter.com/veritynetwork)
- [Telegram](https://t.me/veritynetwork)

---

Built for privacy. Optimized for speed. 🚀
