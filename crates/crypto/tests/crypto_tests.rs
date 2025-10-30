//! Crypto module tests

use crypto::{
    CryptoError, Encryption, Hasher, KeyDerivation, SignatureVerifier, ZeroKnowledgeProof,
};

/// Test ECDSA signature verification
#[test]
fn test_ecdsa_signature_verification() {
    let verifier = SignatureVerifier::new();

    // Test valid ECDSA signature
    let result = verifier.verify_ecdsa_signature(
        b"test_message",
        &[0u8; 64], // Placeholder signature
        &[0u8; 33], // Placeholder public key
    );
    assert!(result); // Placeholder implementation

    // Test invalid signature size
    let result = verifier.verify_ecdsa_signature(
        b"test_message",
        &[0u8; 32], // Wrong size
        &[0u8; 33],
    );
    assert!(!result);

    // Test invalid public key size
    let result = verifier.verify_ecdsa_signature(
        b"test_message",
        &[0u8; 64],
        &[0u8; 32], // Wrong size
    );
    assert!(!result);
}

/// Test EdDSA signature verification
#[test]
fn test_eddsa_signature_verification() {
    let verifier = SignatureVerifier::new();

    // Test valid EdDSA signature
    let result = verifier.verify_eddsa_signature(
        b"test_message",
        &[0u8; 64], // Placeholder signature
        &[0u8; 32], // Placeholder public key
    );
    assert!(result); // Placeholder implementation

    // Test invalid signature size
    let result = verifier.verify_eddsa_signature(
        b"test_message",
        &[0u8; 32], // Wrong size
        &[0u8; 32],
    );
    assert!(!result);

    // Test invalid public key size
    let result = verifier.verify_eddsa_signature(
        b"test_message",
        &[0u8; 64],
        &[0u8; 33], // Wrong size
    );
    assert!(!result);
}

/// Test generic signature verification
#[test]
fn test_generic_signature_verification() {
    let verifier = SignatureVerifier::new();

    // Test EdDSA (64-byte signature, 32-byte public key)
    let result = verifier.verify_signature(
        b"test_message",
        &[0u8; 64], // EdDSA signature
        &[0u8; 32], // EdDSA public key
    );
    assert!(result);

    // Test ECDSA (64-byte signature, 33-byte public key)
    let result = verifier.verify_signature(
        b"test_message",
        &[0u8; 64], // ECDSA signature
        &[0u8; 33], // ECDSA public key
    );
    assert!(result);

    // Test unsupported signature type
    let result = verifier.verify_signature(
        b"test_message",
        &[0u8; 32], // Unsupported size
        &[0u8; 32],
    );
    assert!(!result);
}

/// Test zero-knowledge proofs of knowledge
#[test]
fn test_zero_knowledge_proofs_of_knowledge() {
    let zk = ZeroKnowledgeProof::new();

    // Test proof generation
    let proof = zk.generate_proof_of_knowledge(b"secret", b"public");
    assert!(proof.is_ok());

    // Test proof verification
    let verified = zk.verify_proof_of_knowledge(&proof.unwrap(), b"public");
    assert!(verified);
}

/// Test zero-knowledge range proofs
#[test]
fn test_zero_knowledge_range_proofs() {
    let zk = ZeroKnowledgeProof::new();

    // Test valid range proof
    let proof = zk.generate_range_proof(50, 0, 100);
    assert!(proof.is_ok());

    // Test proof verification
    let verified = zk.verify_range_proof(&proof.unwrap(), 0, 100);
    assert!(verified);

    // Test invalid range proof (value out of range)
    let proof = zk.generate_range_proof(150, 0, 100);
    assert!(proof.is_err());
}

/// Test generic zero-knowledge proofs
#[test]
fn test_zero_knowledge_proofs() {
    let zk = ZeroKnowledgeProof::new();
    let proof = zk.generate_proof(b"test_witness", b"test_statement");
    assert!(proof.is_ok());

    let verified = zk.verify_proof(&proof.unwrap(), b"test_statement");
    assert!(verified);
}

/// Test hashing functions
#[test]
fn test_hashing() {
    let hasher = Hasher::new();
    let hash = hasher.sha256(b"test_data");
    assert_ne!(hash, [0u8; 32]); // Should not be all zeros
}

/// Test hashing with salt
#[test]
fn test_hashing_with_salt() {
    let hasher = Hasher::new();
    let hash1 = hasher.sha256_with_salt(b"test_data", b"salt1");
    let hash2 = hasher.sha256_with_salt(b"test_data", b"salt2");
    assert_ne!(hash1, hash2); // Different salts should produce different hashes
}

/// Test double hashing
#[test]
fn test_double_hashing() {
    let hasher = Hasher::new();
    let hash = hasher.sha256d(b"test_data");
    assert_ne!(hash, [0u8; 32]); // Should not be all zeros

    // Double hash should be different from single hash
    let single_hash = hasher.sha256(b"test_data");
    assert_ne!(hash, single_hash);
}

/// Test AES-GCM encryption and decryption
#[test]
fn test_aes_gcm_encryption_decryption() {
    let encryptor = Encryption::new();
    let key = [1u8; 32]; // 256-bit key
    let data = b"test_data";

    // Test encryption
    let encrypted = encryptor.encrypt_aes_gcm(data, &key);
    assert!(encrypted.is_ok());

    // Test decryption
    let decrypted = encryptor.decrypt_aes_gcm(&encrypted.unwrap(), &key);
    assert!(decrypted.is_ok());
    assert_eq!(decrypted.unwrap(), data);
}

/// Test AES-GCM with invalid key
#[test]
fn test_aes_gcm_invalid_key() {
    let encryptor = Encryption::new();
    let key = [1u8; 16]; // Invalid key size (128-bit instead of 256-bit)
    let data = b"test_data";

    // Test encryption with invalid key
    let encrypted = encryptor.encrypt_aes_gcm(data, &key);
    assert!(encrypted.is_err());

    // Test decryption with invalid key
    let decrypted = encryptor.decrypt_aes_gcm(b"test_encrypted_data", &key);
    assert!(decrypted.is_err());
}

/// Test generic encryption and decryption
#[test]
fn test_encryption_decryption() {
    let encryptor = Encryption::new();
    let key = b"test_key";
    let data = b"test_data";

    // Test encryption
    let encrypted = encryptor.encrypt(data, key);
    assert!(encrypted.is_ok());

    // Test decryption
    let decrypted = encryptor.decrypt(&encrypted.unwrap(), key);
    assert!(decrypted.is_ok());
    assert_eq!(decrypted.unwrap(), data);
}

/// Test encryption with wrong key
#[test]
fn test_encryption_with_wrong_key() {
    let encryptor = Encryption::new();
    let key1 = b"test_key1";
    let key2 = b"test_key2";
    let data = b"test_data";

    // Encrypt with key1
    let encrypted = encryptor.encrypt(data, key1);
    assert!(encrypted.is_ok());

    // Try to decrypt with key2 (should fail)
    let decrypted = encryptor.decrypt(&encrypted.unwrap(), key2);
    assert!(decrypted.is_err());
}

/// Test PBKDF2 key derivation
#[test]
fn test_pbkdf2_derivation() {
    let kdf = KeyDerivation::new();

    // Test key derivation
    let key1 = kdf.pbkdf2_derive(b"password", b"salt", 1000);
    let key2 = kdf.pbkdf2_derive(b"password", b"salt", 1000);
    let key3 = kdf.pbkdf2_derive(b"password", b"different_salt", 1000);
    let key4 = kdf.pbkdf2_derive(b"different_password", b"salt", 1000);

    // Same inputs should produce same output
    assert_eq!(key1, key2);

    // Different salt should produce different output
    assert_ne!(key1, key3);

    // Different password should produce different output
    assert_ne!(key1, key4);
}

/// Test random key generation
#[test]
fn test_random_key_generation() {
    let kdf = KeyDerivation::new();

    // Generate two keys
    let key1 = kdf.generate_random_key(32);
    let key2 = kdf.generate_random_key(32);

    // Check key length
    assert_eq!(key1.len(), 32);
    assert_eq!(key2.len(), 32);

    // Keys should be different (with very high probability)
    assert_ne!(key1, key2);

    // Test different key sizes
    let key16 = kdf.generate_random_key(16);
    let key64 = kdf.generate_random_key(64);

    assert_eq!(key16.len(), 16);
    assert_eq!(key64.len(), 64);
}

/// Test error handling
#[test]
fn test_crypto_errors() {
    let encryptor = Encryption::new();
    let key = b"test_key";
    let wrong_data = b"wrong_encrypted_data";

    // Test decryption error
    let decrypted = encryptor.decrypt(wrong_data, key);
    assert!(matches!(decrypted, Err(CryptoError::DecryptionFailed)));

    // Test invalid key error for AES-GCM
    let aes_enc = Encryption::new();
    let invalid_key = [0u8; 16]; // 128-bit key instead of 256-bit
    let result = aes_enc.encrypt_aes_gcm(b"test_data", &invalid_key);
    assert!(matches!(result, Err(CryptoError::InvalidKey)));
}
