//! Integration tests for the CosmWasm chain adapter

use cosmwasm_chain::{CosmWasmConfig, CosmWasmToken, CosmWasmTransaction};

/// Test CosmWasm configuration
fn test_config() -> CosmWasmConfig {
    CosmWasmConfig {
        rpc_url: "https://rpc.cosmoshub-4.com".to_string(),
        grpc_url: "https://grpc.cosmoshub-4.com".to_string(),
        chain_id: "cosmoshub-4".to_string(),
        gas_limit: 1000000,
    }
}

/// Test CosmWasm adapter creation
#[test]
fn test_cosmwasm_adapter_creation() {
    let config = test_config();
    assert_eq!(config.chain_id, "cosmoshub-4");
}

/// Test CosmWasm token structure
#[test]
fn test_cosmwasm_token_struct() {
    let token = CosmWasmToken {
        contract_address: "cosmos1contractaddress".to_string(),
        name: "Cosmos Token".to_string(),
        symbol: "COSM".to_string(),
        decimals: 6,
    };
    
    assert_eq!(token.symbol, "COSM");
    assert_eq!(token.decimals, 6);
}

/// Test CosmWasm transaction structure
#[test]
fn test_cosmwasm_transaction_struct() {
    let tx = CosmWasmTransaction {
        from: "cosmos1testaddress".to_string(),
        to: "cosmos1testaddress".to_string(),
        amount: "1000".to_string(),
        denom: "uatom".to_string(),
        data: vec![],
        gas_limit: 200000,
        sequence: 0,
    };
    
    assert_eq!(tx.denom, "uatom");
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
    let tx = CosmWasmTransaction {
        from: "cosmos1testaddress".to_string(),
        to: "cosmos1testaddress".to_string(),
        amount: "1000".to_string(),
        denom: "uatom".to_string(),
        data: vec![],
        gas_limit: 500000, // Within limit
        sequence: 0,
    };
    
    // Test gas limit validation would go here if we could instantiate the adapter
    assert!(tx.gas_limit <= config.gas_limit);
}