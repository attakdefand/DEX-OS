//! Security module tests

use security::{Authentication, Authorization, KeyManager, AuthSession};

/// Test authentication functionality
#[test]
fn test_authentication() {
    let mut auth = Authentication::new();
    let result = auth.register_user("test_user", "test_password");
    assert!(result.is_ok());
    
    let session = auth.authenticate("test_user", "test_password");
    assert!(session.is_ok());
}

/// Test authorization functionality
#[test]
fn test_authorization() {
    let mut authz = Authorization::new();
    authz.grant_permission("test_user", "read");
    let result = authz.check_permission("test_user", "read");
    assert!(result.is_ok());
    assert!(result.unwrap());
}

/// Test key management
#[test]
fn test_key_management() {
    let mut key_manager = KeyManager::new();
    
    // Test key generation
    let key = key_manager.generate_key("test_key");
    assert!(key.is_ok());
    
    // Test key retrieval
    let retrieved_key = key_manager.get_key("test_key");
    assert!(retrieved_key.is_some());
    
    // Test key deletion
    let deleted = key_manager.delete_key("test_key");
    assert!(deleted);
    
    // Test non-existent key retrieval
    let non_existent = key_manager.get_key("non_existent");
    assert!(non_existent.is_none());
    
    // Test key listing
    let _ = key_manager.generate_key("key1");
    let _ = key_manager.generate_key("key2");
    let keys = key_manager.list_keys();
    assert!(keys.contains(&"key1".to_string()));
    assert!(keys.contains(&"key2".to_string()));
}

/// Test session management
#[test]
fn test_session_management() {
    let session = AuthSession::new("test_user".to_string());
    
    // Test session creation
    assert!(!session.token().is_empty());
    
    // Test session expiration (placeholder - would need to mock time for real test)
    assert!(!session.is_expired());
    
    // Test user retrieval
    assert_eq!(session.user(), "test_user");
}

/// Test user registration and authentication flow
#[test]
fn test_user_registration_and_authentication_flow() {
    let mut auth = Authentication::new();
    
    // Register a new user
    let register_result = auth.register_user("alice", "secure_password123");
    assert!(register_result.is_ok());
    
    // Try to register the same user again (should fail)
    let register_result = auth.register_user("alice", "different_password");
    assert!(register_result.is_err());
    
    // Authenticate with correct credentials
    let auth_result = auth.authenticate("alice", "secure_password123");
    assert!(auth_result.is_ok());
    
    // Authenticate with incorrect credentials
    let auth_result = auth.authenticate("alice", "wrong_password");
    assert!(auth_result.is_err());
    
    // Authenticate with non-existent user
    let auth_result = auth.authenticate("bob", "any_password");
    assert!(auth_result.is_err());
}

/// Test authorization permissions management
#[test]
fn test_authorization_permissions_management() {
    let mut authz = Authorization::new();
    
    // Grant permissions
    authz.grant_permission("alice", "read");
    authz.grant_permission("alice", "write");
    authz.grant_permission("bob", "read");
    
    // Check permissions
    assert!(authz.check_permission("alice", "read").unwrap());
    assert!(authz.check_permission("alice", "write").unwrap());
    assert!(!authz.check_permission("alice", "delete").unwrap());
    assert!(authz.check_permission("bob", "read").unwrap());
    assert!(!authz.check_permission("bob", "write").unwrap());
    
    // Revoke a permission
    authz.revoke_permission("alice", "write");
    assert!(!authz.check_permission("alice", "write").unwrap());
    
    // Check permissions for non-existent user
    assert!(!authz.check_permission("charlie", "read").unwrap());
    
    // Get user permissions
    let alice_permissions = authz.get_user_permissions("alice");
    assert_eq!(alice_permissions.len(), 1);
    assert!(alice_permissions.contains(&"read".to_string()));
    
    let bob_permissions = authz.get_user_permissions("bob");
    assert_eq!(bob_permissions.len(), 1);
    assert!(bob_permissions.contains(&"read".to_string()));
    
    let charlie_permissions = authz.get_user_permissions("charlie");
    assert_eq!(charlie_permissions.len(), 0);
}

/// Test key derivation functionality
#[test]
fn test_key_derivation() {
    let key_manager = KeyManager::new();
    
    // Test deriving keys from passwords
    let key1 = key_manager.derive_key_from_password("password123", "salt123");
    let key2 = key_manager.derive_key_from_password("password123", "salt123");
    let key3 = key_manager.derive_key_from_password("password123", "different_salt");
    let key4 = key_manager.derive_key_from_password("different_password", "salt123");
    
    // Same password and salt should produce same key
    assert_eq!(key1, key2);
    
    // Different salt should produce different key
    assert_ne!(key1, key3);
    
    // Different password should produce different key
    assert_ne!(key1, key4);
    
    // Keys should be 32 bytes long
    assert_eq!(key1.len(), 32);
    assert_eq!(key2.len(), 32);
    assert_eq!(key3.len(), 32);
    assert_eq!(key4.len(), 32);
}