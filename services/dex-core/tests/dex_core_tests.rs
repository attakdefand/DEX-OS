//! Comprehensive tests for the DEX Core service

use dex_core::{DEXCore, DEXError, Order, OrderSide, OrderType, Token, TokenPair};
use ethereum_types::{Address, U256};
use std::str::FromStr;

/// Create test tokens
fn create_test_tokens() -> (Token, Token) {
    let token_a = Token {
        address: Address::from_str("0x1f9840a85d5af5bf1d1762f925bdaddc4201f984").unwrap(),
        name: "Uniswap".to_string(),
        symbol: "UNI".to_string(),
        decimals: 18,
    };

    let token_b = Token {
        address: Address::from_str("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48").unwrap(),
        name: "USD Coin".to_string(),
        symbol: "USDC".to_string(),
        decimals: 6,
    };

    (token_a, token_b)
}

/// Create test users
fn create_test_users() -> (Address, Address) {
    let user_a = Address::from_str("0xd8da6bf26964af9d7eed9e03e53415d37aa96045").unwrap();
    let user_b = Address::from_str("0x5a1f8e37ea381e1121854199126353b327f02b32").unwrap();
    (user_a, user_b)
}

/// Test DEX Core initialization
#[test]
fn test_dex_core_initialization() {
    let dex = DEXCore::new();
    assert_eq!(dex.liquidity_pools.len(), 0);
    assert_eq!(dex.order_books.len(), 0);
    assert_eq!(dex.tokens.len(), 0);
}

/// Test token management
#[test]
fn test_token_management() {
    let mut dex = DEXCore::new();
    let (token_a, token_b) = create_test_tokens();

    // Add tokens
    dex.add_token(token_a.clone());
    dex.add_token(token_b.clone());

    // Retrieve tokens
    let retrieved_a = dex.get_token(token_a.address).unwrap();
    let retrieved_b = dex.get_token(token_b.address).unwrap();

    assert_eq!(retrieved_a.symbol, "UNI");
    assert_eq!(retrieved_b.symbol, "USDC");

    // Try to retrieve non-existent token
    let non_existent = Address::from_str("0x0000000000000000000000000000000000000000").unwrap();
    let result = dex.get_token(non_existent);
    assert!(result.is_none());
}

/// Test liquidity pool creation
#[test]
fn test_liquidity_pool_creation() {
    let mut dex = DEXCore::new();
    let (token_a, token_b) = create_test_tokens();

    // Create liquidity pool
    let result = dex.create_liquidity_pool(token_a.address, token_b.address, 30);
    assert!(result.is_ok());

    // Try to create duplicate pool
    let result = dex.create_liquidity_pool(token_a.address, token_b.address, 30);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        DEXError::LiquidityPoolError(_)
    ));

    // Verify pool exists
    let pool = dex.get_liquidity_pool(token_a.address, token_b.address);
    assert!(pool.is_some());

    // Verify corresponding order book exists
    let token_pair = TokenPair::new(token_a.address, token_b.address);
    let order_book = dex.get_order_book(token_pair);
    assert!(order_book.is_some());
}

/// Test liquidity operations
#[test]
fn test_liquidity_operations() {
    let mut dex = DEXCore::new();
    let (token_a, token_b) = create_test_tokens();

    // Create pool
    dex.create_liquidity_pool(token_a.address, token_b.address, 30)
        .unwrap();

    // Add liquidity
    let liquidity = dex.add_liquidity(
        token_a.address,
        token_b.address,
        U256::from(1000000u64),
        U256::from(2000000u64),
    );
    assert!(liquidity.is_ok());
    let liquidity_amount = liquidity.unwrap();
    assert!(liquidity_amount > U256::zero());

    // Add more liquidity
    let liquidity2 = dex.add_liquidity(
        token_a.address,
        token_b.address,
        U256::from(500000u64),
        U256::from(1000000u64),
    );
    assert!(liquidity2.is_ok());
    let liquidity_amount2 = liquidity2.unwrap();
    assert!(liquidity_amount2 > U256::zero());

    // Remove liquidity
    let removed = dex.remove_liquidity(token_a.address, token_b.address, liquidity_amount);
    assert!(removed.is_ok());
    let (amount_a, amount_b) = removed.unwrap();
    assert!(amount_a > U256::zero());
    assert!(amount_b > U256::zero());

    // Try to remove more liquidity than available
    let removed = dex.remove_liquidity(
        token_a.address,
        token_b.address,
        U256::from(1000000000u64), // Excessive amount
    );
    assert!(removed.is_err());
    assert!(matches!(
        removed.unwrap_err(),
        DEXError::InsufficientLiquidity(_)
    ));
}

/// Test token swapping
#[test]
fn test_token_swapping() {
    let mut dex = DEXCore::new();
    let (token_a, token_b) = create_test_tokens();

    // Create pool and add liquidity
    dex.create_liquidity_pool(token_a.address, token_b.address, 30)
        .unwrap();
    dex.add_liquidity(
        token_a.address,
        token_b.address,
        U256::from(1000000u64),
        U256::from(1000000u64),
    )
    .unwrap();

    // Perform swap
    let amount_out = dex.swap_tokens(
        token_a.address,
        token_b.address,
        U256::from(1000u64),
        U256::from(900u64),
    );
    assert!(amount_out.is_ok());
    let received = amount_out.unwrap();
    assert!(received > U256::from(900u64));
    assert!(received < U256::from(1000u64)); // Should be less due to fees

    // Try swap with insufficient minimum
    let amount_out = dex.swap_tokens(
        token_a.address,
        token_b.address,
        U256::from(1000u64),
        U256::from(1000u64), // Too high minimum
    );
    assert!(amount_out.is_err());
    assert!(matches!(
        amount_out.unwrap_err(),
        DEXError::InsufficientLiquidity(_)
    ));

    // Try swap with non-existent pool
    let token_c = Address::from_str("0x0000000000000000000000000000000000000001").unwrap();
    let amount_out = dex.swap_tokens(
        token_a.address,
        token_c,
        U256::from(1000u64),
        U256::from(1u64),
    );
    assert!(amount_out.is_err());
    assert!(matches!(
        amount_out.unwrap_err(),
        DEXError::InvalidTokenPair(_)
    ));
}

/// Test order placement
#[test]
fn test_order_placement() {
    let mut dex = DEXCore::new();
    let (token_a, token_b) = create_test_tokens();
    let (user_a, _) = create_test_users();

    // Create pool and order book
    dex.create_liquidity_pool(token_a.address, token_b.address, 30)
        .unwrap();

    // Create order
    let token_pair = TokenPair::new(token_a.address, token_b.address);
    let order = Order::new(
        "order1".to_string(),
        user_a,
        token_pair.clone(),
        OrderSide::Buy,
        OrderType::Limit,
        U256::from(100u64),
        U256::from(500u64),
        1234567890,
    );

    // Place order
    let result = dex.place_order(order);
    assert!(result.is_ok());

    // Try to place order for non-existent order book
    let token_c = Address::from_str("0x0000000000000000000000000000000000000001").unwrap();
    let token_pair_invalid = TokenPair::new(token_a.address, token_c);
    let order_invalid = Order::new(
        "order2".to_string(),
        user_a,
        token_pair_invalid,
        OrderSide::Buy,
        OrderType::Limit,
        U256::from(100u64),
        U256::from(500u64),
        1234567890,
    );

    let result = dex.place_order(order_invalid);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), DEXError::OrderBookError(_)));
}

/// Test order cancellation
#[test]
fn test_order_cancellation() {
    let mut dex = DEXCore::new();
    let (token_a, token_b) = create_test_tokens();
    let (user_a, _) = create_test_users();

    // Create pool and order book
    dex.create_liquidity_pool(token_a.address, token_b.address, 30)
        .unwrap();

    // Create and place order
    let token_pair = TokenPair::new(token_a.address, token_b.address);
    let order = Order::new(
        "order1".to_string(),
        user_a,
        token_pair.clone(),
        OrderSide::Buy,
        OrderType::Limit,
        U256::from(100u64),
        U256::from(500u64),
        1234567890,
    );

    dex.place_order(order).unwrap();

    // Cancel order
    let cancelled = dex.cancel_order(token_pair.clone(), "order1");
    assert!(cancelled.is_ok());
    assert!(cancelled.unwrap().is_some());

    // Try to cancel non-existent order
    let cancelled = dex.cancel_order(token_pair, "nonexistent");
    assert!(cancelled.is_ok());
    assert!(cancelled.unwrap().is_none());
}

/// Test order matching
#[test]
fn test_order_matching() {
    let mut dex = DEXCore::new();
    let (token_a, token_b) = create_test_tokens();
    let (user_a, user_b) = create_test_users();

    // Create pool and order book
    dex.create_liquidity_pool(token_a.address, token_b.address, 30)
        .unwrap();

    // Create token pair
    let token_pair = TokenPair::new(token_a.address, token_b.address);

    // Place buy order
    let buy_order = Order::new(
        "buy1".to_string(),
        user_a,
        token_pair.clone(),
        OrderSide::Buy,
        OrderType::Limit,
        U256::from(100u64),
        U256::from(500u64),
        1234567890,
    );

    // Place sell order
    let sell_order = Order::new(
        "sell1".to_string(),
        user_b,
        token_pair.clone(),
        OrderSide::Sell,
        OrderType::Limit,
        U256::from(95u64),
        U256::from(500u64),
        1234567891,
    );

    dex.place_order(buy_order).unwrap();
    dex.place_order(sell_order).unwrap();

    // Match orders
    let matches = dex.match_orders(token_pair);
    assert!(matches.is_ok());
    let matches = matches.unwrap();
    assert_eq!(matches.len(), 1);

    let (matched_buy, matched_sell, amount, price) = &matches[0];
    assert_eq!(*amount, U256::from(500u64));
    assert_eq!(*price, U256::from(95u64)); // Seller's price
    assert_eq!(matched_buy.id, "buy1");
    assert_eq!(matched_sell.id, "sell1");
}

/// Test security integration
#[test]
fn test_security_integration() {
    // Test that security modules can be instantiated
    use crypto::SignatureVerifier;
    use security::{Authorization, KeyManager};

    let _key_manager = KeyManager::new();
    let _authorization = Authorization::new();
    let _signature_verifier = SignatureVerifier::new();

    // These should compile and instantiate without errors
    assert!(true);
}

/// Test edge cases
#[test]
fn test_edge_cases() {
    let mut dex = DEXCore::new();
    let (token_a, token_b) = create_test_tokens();

    // Test zero amount swaps
    dex.create_liquidity_pool(token_a.address, token_b.address, 30)
        .unwrap();
    dex.add_liquidity(
        token_a.address,
        token_b.address,
        U256::from(1000u64),
        U256::from(1000u64),
    )
    .unwrap();

    let result = dex.swap_tokens(token_a.address, token_b.address, U256::zero(), U256::zero());
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), DEXError::InvalidAmount(_)));

    // Test swap with insufficient liquidity
    let result = dex.swap_tokens(
        token_a.address,
        token_b.address,
        U256::from(10000000u64), // Very large amount
        U256::from(1u64),
    );
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        DEXError::InsufficientLiquidity(_)
    ));
}

/// Test active pairs retrieval
#[test]
fn test_active_pairs_retrieval() {
    let mut dex = DEXCore::new();
    let (token_a, token_b) = create_test_tokens();

    // Initially no active pairs
    let pairs = dex.get_active_pairs();
    assert_eq!(pairs.len(), 0);

    // Create pool
    dex.create_liquidity_pool(token_a.address, token_b.address, 30)
        .unwrap();

    // Now should have one active pair
    let pairs = dex.get_active_pairs();
    assert_eq!(pairs.len(), 1);

    let token_pair = TokenPair::new(token_a.address, token_b.address);
    assert_eq!(pairs[0], token_pair);
}
