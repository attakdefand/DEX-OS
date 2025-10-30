//! SVM Programs for DEX-OS
//!
//! This crate provides SVM programs for the DEX-OS.

/// SVM programs initialization
pub fn init() {
    println!("SVM programs initialized");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
