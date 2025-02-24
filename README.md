# RustedBridge - Cross-Chain Bridge

A secure cross-chain bridge implementation between BASE and Ethereum, following the ERC-7683 standard. This bridge enables secure asset transfers between the Ethereum and Base network. check1

## Features

- ERC-7683 compliant implementation
- Bidirectional transfers between BASE and Ethereum
- Secure message verification and validation
- Configurable confirmation blocks
- Event monitoring and processing
- Comprehensive error handling
- Logging and monitoring

## Prerequisites

- Rust 1.70 or higher
- Access to Ethereum and Base RPC endpoints
- Valid private key for transaction signing
- Smart contracts deployed on both networks

## Setup

1. Clone the repository:
```bash
git clone https://github.com/yourusername/rusted-bridge
cd rusted-bridge
```

2. Create a `.env` file with your configuration:
```env
ETHEREUM_RPC=your_ethereum_rpc_url
BASE_RPC=your_base_rpc_url
BRIDGE_CONTRACT_ETHEREUM=ethereum_contract_address
BRIDGE_CONTRACT_BASE=base_contract_address
VALIDATOR_PRIVATE_KEY=your_private_key
ETHEREUM_CHAIN_ID=1
BASE_CHAIN_ID=8453
CONFIRMATION_BLOCKS=12
```

3. Build the project:
```bash
cargo build --release
```

4. Run the bridge:
```bash
cargo run --release
```

## Architecture

The bridge consists of several key components:

- **Bridge Handler**: Core logic for processing cross-chain transfers
- **Message Verification**: Implementation of ERC-7683 message verification
- **Event Monitoring**: Concurrent monitoring of events on both chains
- **Configuration Management**: Environment-based configuration
- **Error Handling**: Comprehensive error handling and logging

## Security Considerations

- Private keys should be properly secured
- Use secure RPC endpoints
- Monitor gas prices and transaction costs
- Implement proper transaction retry mechanisms
- Regular security audits recommended

## Contributing

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Create a new Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Disclaimer

This is a reference implementation. Use at your own risk. Always perform security audits before deploying to production. 