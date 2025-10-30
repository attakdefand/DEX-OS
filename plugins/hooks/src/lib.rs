//! Hooks Plugin for DEX-OS
//!
//! This crate provides WASM hooks for the DEX-OS.

/// Hooks plugin initialization
pub fn init() {
    println!("Hooks plugin initialized");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
