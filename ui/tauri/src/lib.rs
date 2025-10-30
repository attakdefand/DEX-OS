//! Tauri Desktop App for DEX-OS
//!
//! This crate provides the desktop application for the DEX-OS.

/// Tauri app initialization
pub fn init() {
    println!("Tauri app initialized");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
