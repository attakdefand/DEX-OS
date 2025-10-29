# DEX-OS Project Structure

## Overview

DEX-OS is a decentralized exchange operating system built with Rust. The project is organized into several key components:

## Directory Structure

```
dex-os/
├── Cargo.toml                 # Workspace configuration
├── rust-toolchain.toml        # Rust toolchain specification
├── README.md                  # Project documentation
├── Makefile                   # Build automation
├── docker-compose.yml         # Local development environment
├── .github/workflows/         # CI/CD configuration
├── crates/                    # Shared libraries
│   ├── security/              # Security layer
│   ├── crypto/                # Cryptography utilities
│   └── p2p/                   # P2P networking
├── kernel/                    # Core OS kernel
├── chains/                    # Blockchain adapters
│   ├── evm/                   # Ethereum Virtual Machine
│   ├── svm/                   # Solana Virtual Machine
│   ├── cosmwasm/              # CosmWasm contracts
│   └── ibc/                   # Inter-blockchain communication
├── plugins/                   # Extensible modules
├── services/                  # Off-chain services
├── ui/                        # User interfaces
│   ├── pwa/                   # Progressive web app
│   ├── tauri/                 # Desktop application
│   ├── ai-chat/               # AI trading assistant
│   └── android/               # Android application
├── tests/                     # Test suite
└── infra/                     # Infrastructure
    ├── docker/                # Docker configurations
    ├── k8s/                   # Kubernetes manifests
    └── offline/               # Air-gapped tools
```

## Component Descriptions

### Crates (Shared Libraries)
- **security**: Authentication, authorization, and protection mechanisms
- **crypto**: Zero-knowledge proofs, signature verification, and hashing
- **p2p**: Libp2p utilities for peer-to-peer networking

### Kernel
The core of the operating system, responsible for:
- Virtual machine execution
- Consensus mechanisms
- Resource management

### Chains
Blockchain-specific adapters:
- **EVM**: Ethereum-compatible chains
- **SVM**: Solana-compatible chains
- **CosmWasm**: Cosmos-compatible chains
- **IBC**: Cross-chain communication

### Plugins
Extensible modules:
- **hooks**: WASM-based extension points
- **ai**: AI-powered trading and fee optimization
- **mev**: MEV protection and private mempool

### Services
Off-chain microservices:
- **p2p**: Libp2p node
- **indexer-zk**: ZK-rollup event processing
- **api**: GraphQL and REST APIs
- **bundler-zk**: ERC-4337 account abstraction bundler
- **security-monitor**: Anomaly detection

### UI
User interfaces:
- **pwa**: Web-based interface
- **tauri**: Desktop application
- **ai-chat**: AI trading assistant
- **android**: Mobile application

### Tests
Comprehensive testing framework with:
- Unit tests
- Integration tests
- Security tests
- Performance tests

### Infra
Deployment and infrastructure:
- **docker**: Container configurations
- **k8s**: Kubernetes manifests
- **offline**: Air-gapped synchronization tools

## Development Workflow

1. **Setup**: Install Rust 1.80+ and Docker
2. **Build**: Run `make build` or `cargo build`
3. **Test**: Run `make test` or `cargo test`
4. **Run**: Use `docker-compose up` for local development
5. **Deploy**: Follow guides in `infra/` directory

## Android Deployment

For Android deployment:
1. Build with `make build-android`
2. Install NDK and set up environment
3. Use Android Studio to package and deploy
4. Follow instructions in `infra/android-installation.md`