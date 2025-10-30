//! Security module for DEX-OS
//!
//! This crate provides protection layers for the DEX-OS including authentication,
//! authorization, and key management.

use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Authentication manager
pub struct Authentication {
    users: HashMap<String, UserCredentials>,
}

/// User credentials
#[derive(Debug, Clone)]
pub struct UserCredentials {
    username: String,
    password_hash: String,
    salt: String,
}

impl Authentication {
    /// Create a new authentication manager
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
        }
    }

    /// Register a new user
    pub fn register_user(&mut self, username: &str, password: &str) -> Result<(), SecurityError> {
        if self.users.contains_key(username) {
            return Err(SecurityError::AuthenticationFailed);
        }

        let salt = self.generate_salt();
        let password_hash = self.hash_password(password, &salt);

        let credentials = UserCredentials {
            username: username.to_string(),
            password_hash,
            salt,
        };

        self.users.insert(username.to_string(), credentials);
        Ok(())
    }

    /// Authenticate a user with credentials
    pub fn authenticate(
        &self,
        username: &str,
        password: &str,
    ) -> Result<AuthSession, SecurityError> {
        let credentials = self
            .users
            .get(username)
            .ok_or(SecurityError::AuthenticationFailed)?;

        let password_hash = self.hash_password(password, &credentials.salt);
        if password_hash == credentials.password_hash {
            let session = AuthSession::new(username.to_string());
            Ok(session)
        } else {
            Err(SecurityError::AuthenticationFailed)
        }
    }

    /// Verify a session token
    pub fn verify_session(&self, token: &str) -> Result<bool, SecurityError> {
        // In a real implementation, this would verify the token against stored sessions
        // For now, we'll just check if it's a valid token format
        Ok(token.starts_with("token_"))
    }

    /// Hash a password with a salt
    fn hash_password(&self, password: &str, salt: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(password);
        hasher.update(salt);
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    /// Generate a random salt
    fn generate_salt(&self) -> String {
        // In a real implementation, this would generate a secure random salt
        // For now, we'll use a placeholder
        "salt_1234567890".to_string()
    }
}

/// Authorization manager
pub struct Authorization {
    permissions: HashMap<String, Vec<String>>,
}

impl Authorization {
    /// Create a new authorization manager
    pub fn new() -> Self {
        Self {
            permissions: HashMap::new(),
        }
    }

    /// Grant a permission to a user
    pub fn grant_permission(&mut self, user: &str, permission: &str) {
        let user_permissions = self
            .permissions
            .entry(user.to_string())
            .or_insert_with(Vec::new);
        if !user_permissions.contains(&permission.to_string()) {
            user_permissions.push(permission.to_string());
        }
    }

    /// Revoke a permission from a user
    pub fn revoke_permission(&mut self, user: &str, permission: &str) {
        if let Some(user_permissions) = self.permissions.get_mut(user) {
            user_permissions.retain(|p| p != permission);
        }
    }

    /// Check if a user has permission to perform an action
    pub fn check_permission(&self, user: &str, action: &str) -> Result<bool, SecurityError> {
        if let Some(user_permissions) = self.permissions.get(user) {
            Ok(user_permissions.contains(&action.to_string()))
        } else {
            Ok(false)
        }
    }

    /// Get all permissions for a user
    pub fn get_user_permissions(&self, user: &str) -> Vec<String> {
        self.permissions.get(user).cloned().unwrap_or_else(Vec::new)
    }
}

/// Key manager for cryptographic keys
pub struct KeyManager {
    keys: HashMap<String, Vec<u8>>,
}

impl KeyManager {
    /// Create a new key manager
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
        }
    }

    /// Generate a new key
    pub fn generate_key(&mut self, key_id: &str) -> Result<Vec<u8>, SecurityError> {
        // In a real implementation, this would generate a secure random key
        // For now, we'll generate a more realistic placeholder
        let mut key = Vec::new();
        for i in 0..32 {
            key.push((i as u8) ^ 0x55); // XOR with a constant for variation
        }
        self.keys.insert(key_id.to_string(), key.clone());
        Ok(key)
    }

    /// Generate a key from a password
    pub fn derive_key_from_password(&self, password: &str, salt: &str) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(password);
        hasher.update(salt);
        let result = hasher.finalize();
        result.to_vec()
    }

    /// Get a key by ID
    pub fn get_key(&self, key_id: &str) -> Option<&Vec<u8>> {
        self.keys.get(key_id)
    }

    /// Delete a key
    pub fn delete_key(&mut self, key_id: &str) -> bool {
        self.keys.remove(key_id).is_some()
    }

    /// List all key IDs
    pub fn list_keys(&self) -> Vec<String> {
        self.keys.keys().cloned().collect()
    }
}

/// Authentication session
pub struct AuthSession {
    #[allow(dead_code)]
    user: String,
    token: String,
    #[allow(dead_code)]
    created_at: u64,
    expires_at: u64,
}

impl AuthSession {
    /// Create a new session
    pub fn new(user: String) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        Self {
            user,
            token: format!("token_{}", Self::generate_secure_token()),
            created_at: now,
            expires_at: now + 3600, // 1 hour expiration
        }
    }

    /// Generate a secure token (placeholder implementation)
    fn generate_secure_token() -> String {
        // In a real implementation, this would generate a cryptographically secure token
        // For now, we'll use a placeholder
        "secure_token_1234567890abcdef".to_string()
    }

    /// Get the session token
    pub fn token(&self) -> &str {
        &self.token
    }

    /// Check if the session is expired
    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        now > self.expires_at
    }

    /// Get the username associated with this session
    pub fn user(&self) -> &str {
        &self.user
    }
}

/// Security error types
#[derive(Debug)]
pub enum SecurityError {
    AuthenticationFailed,
    AuthorizationFailed,
    KeyGenerationFailed,
    InvalidToken,
    UserAlreadyExists,
}

impl std::fmt::Display for SecurityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SecurityError::AuthenticationFailed => write!(f, "Authentication failed"),
            SecurityError::AuthorizationFailed => write!(f, "Authorization failed"),
            SecurityError::KeyGenerationFailed => write!(f, "Key generation failed"),
            SecurityError::InvalidToken => write!(f, "Invalid token"),
            SecurityError::UserAlreadyExists => write!(f, "User already exists"),
        }
    }
}

impl std::error::Error for SecurityError {}

/// Security module initialization
pub fn init() {
    println!("Security module initialized");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_registration() {
        let mut auth = Authentication::new();
        let result = auth.register_user("test_user", "test_password");
        assert!(result.is_ok());
    }

    #[test]
    fn test_duplicate_user_registration() {
        let mut auth = Authentication::new();
        let _ = auth.register_user("test_user", "test_password");
        let result = auth.register_user("test_user", "different_password");
        assert!(result.is_err());
    }

    #[test]
    fn test_successful_authentication() {
        let mut auth = Authentication::new();
        let _ = auth.register_user("test_user", "test_password");
        let session = auth.authenticate("test_user", "test_password");
        assert!(session.is_ok());
    }

    #[test]
    fn test_failed_authentication() {
        let mut auth = Authentication::new();
        let _ = auth.register_user("test_user", "test_password");
        let session = auth.authenticate("test_user", "wrong_password");
        assert!(session.is_err());
    }

    #[test]
    fn test_nonexistent_user_authentication() {
        let auth = Authentication::new();
        let session = auth.authenticate("nonexistent_user", "any_password");
        assert!(session.is_err());
    }

    #[test]
    fn test_authorization_grant_and_check() {
        let mut authz = Authorization::new();
        authz.grant_permission("test_user", "read");
        let result = authz.check_permission("test_user", "read");
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_authorization_without_permission() {
        let authz = Authorization::new();
        let result = authz.check_permission("test_user", "read");
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_authorization_revoke_permission() {
        let mut authz = Authorization::new();
        authz.grant_permission("test_user", "read");
        authz.revoke_permission("test_user", "read");
        let result = authz.check_permission("test_user", "read");
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_key_manager() {
        let mut key_manager = KeyManager::new();
        let key = key_manager.generate_key("test_key");
        assert!(key.is_ok());

        let retrieved_key = key_manager.get_key("test_key");
        assert!(retrieved_key.is_some());

        let deleted = key_manager.delete_key("test_key");
        assert!(deleted);

        let non_existent = key_manager.get_key("non_existent");
        assert!(non_existent.is_none());
    }

    #[test]
    fn test_key_derivation() {
        let key_manager = KeyManager::new();
        let key1 = key_manager.derive_key_from_password("password", "salt");
        let key2 = key_manager.derive_key_from_password("password", "salt");
        let key3 = key_manager.derive_key_from_password("password", "different_salt");

        assert_eq!(key1, key2);
        assert_ne!(key1, key3);
    }

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

    #[test]
    fn test_token_verification() {
        let auth = Authentication::new();
        let result = auth.verify_session("token_valid_token");
        assert!(result.is_ok());
        assert!(result.unwrap());

        let result = auth.verify_session("invalid_token");
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }
}
