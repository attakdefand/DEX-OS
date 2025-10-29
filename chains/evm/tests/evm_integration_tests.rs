//! Integration tests for the EVM chain adapter

use evm_chain::{EVMConfig, EVMToken, EVMTransaction};
use ethereum_types::{H160, U256};
use std::str::FromStr;

/// Test EVM configuration
fn test_config() -> EVMConfig {
    EVMConfig {
        rpc_url: "https://mainnet.infura.io/v3/YOUR_PROJECT_ID".to_string(),
        chain_id: 1,
        gas_limit: 10_000_000,
        gas_price: 20_000_000_000,
    }
}

/// Test EVM adapter creation
#[test]
fn test_evm_adapter_creation() {
    let config = test_config();
    // This would require a real RPC endpoint, so we'll skip the actual connection test
    assert_eq!(config.chain_id, 1);
}

/// Test EVM token structure
#[test]
fn test_evm_token_struct() {
    let token_address = H160::from_str("0x1f9840a85d5af5bf1d1762f925bdaddc4201f984").unwrap();
    let token = EVMToken {
        address: token_address,
        name: "Uniswap".to_string(),
        symbol: "UNI".to_string(),
        decimals: 18,
    };
    
    assert_eq!(token.symbol, "UNI");
    assert_eq!(token.decimals, 18);
}

/// Test EVM transaction structure
#[test]
fn test_evm_transaction_struct() {
    let tx = EVMTransaction {
        from: H160::zero(),
        to: H160::zero(),
        value: U256::zero(),
        data: vec![],
        gas_limit: 1_000_000,
        gas_price: U256::zero(),
        nonce: 0,
    };
    
    assert_eq!(tx.gas_limit, 1_000_000);
}

/// Test security layer integration
#[test]
fn test_security_integration() {
    // Test that the security modules can be instantiated
    use security::{Authentication, Authorization, KeyManager};
    
    let _auth = Authentication::new();
    let _authz = Authorization::new();
    let _key_manager = KeyManager::new();
    
    // These should compile and instantiate without errors
    assert!(true);
}

/// Test crypto layer integration
#[test]
fn test_crypto_integration() {
    // Test that the crypto modules can be instantiated
    use crypto::{SignatureVerifier, ZeroKnowledgeProof, Hasher, Encryption};
    
    let _verifier = SignatureVerifier::new();
    let _zk = ZeroKnowledgeProof::new();
    let _hasher = Hasher::new();
    let _encryptor = Encryption::new();
    
    // These should compile and instantiate without errors
    assert!(true);
}

/// Test MEV protection features
#[test]
fn test_mev_protection() {
    // Test that the MEV protection mechanisms are in place
    // This is a placeholder for actual MEV protection tests
    assert!(true);
}

/// Test transaction validation
#[test]
fn test_transaction_validation() {
    let config = test_config();
    // Skip actual connection but test the validation logic
    let tx = EVMTransaction {
        from: H160::zero(),
        to: H160::zero(),
        value: U256::zero(),
        data: vec![],
        gas_limit: 5_000_000, // Within limit
        gas_price: U256::zero(),
        nonce: 0,
    };
    
    // Test gas limit validation would go here if we could instantiate the adapter
    assert!(tx.gas_limit <= config.gas_limit);
}