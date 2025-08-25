# Dogecoin Adaptation Status

## Overview
This repository (`wonky-ord-dogecoin`) is **already fully adapted for Dogecoin**. It is a fork of the original Bitcoin Ordinals project that has been specifically modified to work with the Dogecoin blockchain.

## Key Adaptations Already in Place

### 1. Package Configuration
- **Package name**: `ord-dogecoin`
- **Description**: "Ordinal wallet and block explorer for dogecoin"
- **Version**: 0.5.1
- **Dependencies**: Uses Dogecoin-specific forks:
  - `bitcoin = { git = "https://github.com/apezord/rust-dogecoin" }`
  - `bitcoincore-rpc = { git = "https://github.com/apezord/rust-dogecoincore-rpc" }`

### 2. Network Configuration
- **RPC Ports**: Uses Dogecoin-specific ports
  - Mainnet: 22555
  - Testnet: 44555
  - Signet: 38332
  - Regtest: 18332
- **Genesis Blocks**: Contains Dogecoin genesis block hashes
- **Network Mapping**: `Network::Bitcoin` maps to Dogecoin mainnet in the Dogecoin fork

### 3. Dogecoin-Specific Features
- **Dunes Support**: Full support for Dogecoin's "dunes" (similar to Bitcoin's runes)
- **DRC-20 Support**: Support for Dogecoin's DRC-20 token standard
- **Inscription Heights**: Configured for Dogecoin-specific heights
  - First inscription height: 4600000 (mainnet)
  - First dune height: 5084000 (mainnet)

### 4. Block Reward System
- **Wonky Block Rewards**: Implements the actual Dogecoin block rewards from block 0 to 144,999
- **Epoch System**: Custom epoch system for Dogecoin's unique reward structure
- **Subsidy Data**: Uses `subsidies.json` and `starting_sats.json` for accurate Dogecoin rewards

### 5. Configuration Files
- **Dogecoin Data Directory**: Supports `.dogecoin` directory structure
- **Cookie File**: Uses Dogecoin Core cookie authentication
- **Environment Variables**: 
  - `SUBSIDIES_PATH`: Path to Dogecoin subsidy data
  - `STARTING_SATS_PATH`: Path to Dogecoin starting satoshis data

## Build Status
✅ **Successfully compiles** with both debug and release profiles
✅ **No compilation errors** - only warnings about unused code
✅ **Binary works correctly** and shows Dogecoin-specific help text

## Available Commands
The adapted version includes all original Ordinals functionality plus Dogecoin-specific features:

### Core Commands
- `index` - Update the Dogecoin index
- `server` - Run the Dogecoin explorer server
- `info` - Display index statistics

### Wallet Commands
- `wallet create` - Create new Dogecoin wallet
- `wallet inscribe` - Create inscriptions on Dogecoin
- `wallet etch` - Create dunes on Dogecoin
- `wallet send` - Send satoshis or inscriptions
- `wallet balance` - Get wallet balance

### Dogecoin-Specific Commands
- `dunes` - List all dunes
- `balances` - List all dune balances
- `epochs` - List the first satoshis of each reward epoch

## Usage Instructions

### Prerequisites
1. **Dogecoin Core Node**: Must be running and fully synced
2. **Dogecoin Core Version**: Tested with v1.14.8
3. **Required Flags**: 
   ```bash
   dogecoind -txindex -rpcuser=foo -rpcpassword=bar -rpcport=22555 -rpcallowip=0.0.0.0/0 -rpcbind=127.0.0.1
   ```

### Environment Setup
```bash
export RUST_LOG=info
export SUBSIDIES_PATH=/path/to/wonky-ord-dogecoin/subsidies.json
export STARTING_SATS_PATH=/path/to/wonky-ord-dogecoin/starting_sats.json
```

### Running the Indexer
```bash
# Start indexing
./target/release/ord --rpc-url=YOUR_RPC_URL --data-dir=/path/to/data --nr-parallel-requests=16 --first-inscription-height=4609723 --first-dune-height=5084000 --index-dunes --index-transactions --index-drc20 index

# Start server
./target/release/ord --rpc-url=YOUR_RPC_URL --data-dir=/path/to/data --nr-parallel-requests=16 --first-inscription-height=4609723 --first-dune-height=5084000 --index-dunes --index-transactions --index-drc20 server
```

## Docker Support
The repository includes Docker configuration for easy deployment:
- **Dockerfile**: Ready to build Dogecoin-adapted image
- **docker-compose.yaml**: Complete deployment setup
- **Environment variables**: All Dogecoin-specific configurations

## Key Differences from Bitcoin Ordinals
1. **Block Rewards**: Uses actual Dogecoin block rewards instead of simplified Bitcoin model
2. **Network**: Maps to Dogecoin network instead of Bitcoin
3. **Features**: Includes Dogecoin-specific features like dunes and DRC-20
4. **Ports**: Uses Dogecoin RPC ports
5. **Data**: Uses Dogecoin-specific subsidy and starting satoshis data

## Conclusion
This repository is **production-ready** for Dogecoin Ordinals. No additional adaptation is needed - it's already a complete Dogecoin implementation of the Ordinals protocol with all the necessary modifications in place.

The code successfully compiles, runs, and provides all the functionality needed for Dogecoin Ordinals, including the unique "wonky" block reward system that makes Dogecoin's ordinal system distinct from Bitcoin's.
