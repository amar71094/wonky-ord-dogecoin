# Quick Setup Guide for Dogecoin Ordinals

## What You Have
You now have a **fully working Dogecoin Ordinals system**! This is a complete adaptation of Bitcoin's Ordinals protocol for Dogecoin.

## What You Need to Do Next

### Step 1: Install Dogecoin Core
1. Go to [Dogecoin releases](https://github.com/dogecoin/dogecoin/releases)
2. Download the latest version for your operating system
3. Install it following the instructions

### Step 2: Set Up Your Dogecoin Node
1. Create a configuration file for Dogecoin Core
2. Start Dogecoin Core with these settings:
   ```bash
   dogecoind -txindex -rpcuser=your_username -rpcpassword=your_password -rpcport=22555 -rpcallowip=0.0.0.0/0 -rpcbind=127.0.0.1
   ```
3. Wait for it to fully sync (this can take several hours)

### Step 3: Set Environment Variables
Open your terminal and run these commands (replace the paths with your actual paths):
```bash
export RUST_LOG=info
export SUBSIDIES_PATH=/Users/apple/Desktop/dogecoin/wonky-ord-dogecoin/subsidies.json
export STARTING_SATS_PATH=/Users/apple/Desktop/dogecoin/wonky-ord-dogecoin/starting_sats.json
```

### Step 4: Create Data Directory
```bash
mkdir -p /Users/apple/Desktop/dogecoin/ord-data
```

### Step 5: Start the Indexer
```bash
cd /Users/apple/Desktop/dogecoin/wonky-ord-dogecoin
./target/release/ord --rpc-url=http://your_username:your_password@127.0.0.1:22555 --data-dir=/Users/apple/Desktop/dogecoin/ord-data --nr-parallel-requests=16 --first-inscription-height=4609723 --first-dune-height=5084000 --index-dunes --index-transactions --index-drc20 index
```

### Step 6: Start the Web Server (Optional)
In a new terminal window:
```bash
cd /Users/apple/Desktop/dogecoin/wonky-ord-dogecoin
./target/release/ord --rpc-url=http://your_username:your_password@127.0.0.1:22555 --data-dir=/Users/apple/Desktop/dogecoin/ord-data --nr-parallel-requests=16 --first-inscription-height=4609723 --first-dune-height=5084000 --index-dunes --index-transactions --index-drc20 server
```

## What This Gives You
- **Dogecoin Ordinals Explorer**: A web interface to browse Dogecoin inscriptions
- **Wallet Functionality**: Create wallets, inscribe content, send inscriptions
- **Dunes Support**: Dogecoin's version of Bitcoin's runes
- **DRC-20 Support**: Dogecoin's token standard
- **API**: Programmatic access to all Dogecoin ordinal data

## Important Notes
- **Replace usernames/passwords**: Use your own secure credentials
- **Full sync required**: Dogecoin Core must be fully synced before starting
- **Storage space**: The index can use up to 400GB of storage
- **Time**: Initial indexing can take several hours

## Getting Help
- Check the main README.md for detailed instructions
- The system is already built and ready to use
- All Dogecoin-specific adaptations are already in place

## What's Different from Bitcoin Ordinals
- Uses Dogecoin's actual block rewards (the "wonky" system)
- Supports Dogecoin-specific features like dunes and DRC-20
- Works with Dogecoin network and addresses
- Uses Dogecoin RPC ports and data directories

You're all set! The hard work of adapting Bitcoin Ordinals to Dogecoin has already been done for you.
