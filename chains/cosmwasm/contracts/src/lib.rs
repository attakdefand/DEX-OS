//! CosmWasm Contracts for DEX-OS
//!
//! This crate provides CosmWasm contracts for the DEX-OS.

/// CosmWasm contracts initialization
pub fn init() {
    println!("CosmWasm contracts initialized");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
