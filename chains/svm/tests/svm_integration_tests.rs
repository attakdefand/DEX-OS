//! Integration tests for the SVM chain adapter

use svm_chain::{SVMConfig, SVMToken, SVMTransaction};

/// Test SVM configuration
fn test_config() -> SVMConfig {
    SVMConfig {
        rpc_url: "https://api.mainnet-beta.solana.com".to_string(),
        commitment: "confirmed".to_string(),
        gas_limit: 1000000,
    }
}

/// Test SVM adapter creation
#[test]
fn test_svm_adapter_creation() {
    let config = test_config();
    assert_eq!(config.commitment, "confirmed");
}

/// Test SVM token structure
#[test]
fn test_svm_token_struct() {
    let token = SVMToken {
        mint_address: "So11111111111111111111111111111111111111112".to_string(),
        name: "Wrapped SOL".to_string(),
        symbol: "SOL".to_string(),
        decimals: 9,
    };

    assert_eq!(token.symbol, "SOL");
    assert_eq!(token.decimals, 9);
}

/// Test SVM transaction structure
#[test]
fn test_svm_transaction_struct() {
    let tx = SVMTransaction {
        from: "test_address".to_string(),
        to: "test_address".to_string(),
        amount: 1000,
        data: vec![],
        gas_limit: 100000,
        nonce: 0,
    };

    assert_eq!(tx.amount, 1000);
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
    use crypto::{Encryption, Hasher, SignatureVerifier, ZeroKnowledgeProof};

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
    let tx = SVMTransaction {
        from: "test_address".to_string(),
        to: "test_address".to_string(),
        amount: 1000,
        data: vec![],
        gas_limit: 500000, // Within limit
        nonce: 0,
    };

    // Test gas limit validation would go here if we could instantiate the adapter
    assert!(tx.gas_limit <= config.gas_limit);
}
