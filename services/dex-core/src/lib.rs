//! DEX Core Service for DEX-OS
//!
//! This crate provides the core decentralized exchange functionality including
//! token swapping, liquidity pools, and order book management.

use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::{debug, info, warn};
use ethereum_types::{Address, U256};

// Import security and crypto modules
use security::{KeyManager, Authorization};
use crypto::{SignatureVerifier, ZeroKnowledgeProof};

/// DEX Core error types
#[derive(Error, Debug)]
pub enum DEXError {
    #[error("Insufficient liquidity: {0}")]
    InsufficientLiquidity(String),
    #[error("Insufficient balance: {0}")]
    InsufficientBalance(String),
    #[error("Invalid token pair: {0}")]
    InvalidTokenPair(String),
    #[error("Invalid amount: {0}")]
    InvalidAmount(String),
    #[error("Security error: {0}")]
    SecurityError(String),
    #[error("Order book error: {0}")]
    OrderBookError(String),
    #[error("Liquidity pool error: {0}")]
    LiquidityPoolError(String),
}

/// Token information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub address: Address,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
}

/// Token pair for trading
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TokenPair {
    pub token_a: Address,
    pub token_b: Address,
}

impl TokenPair {
    pub fn new(token_a: Address, token_b: Address) -> Self {
        // Ensure consistent ordering to avoid duplicate pairs
        if token_a < token_b {
            Self { token_a, token_b }
        } else {
            Self { token_a: token_b, token_b: token_a }
        }
    }
}

/// Liquidity pool information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiquidityPool {
    pub token_pair: TokenPair,
    pub reserve_a: U256,
    pub reserve_b: U256,
    pub total_liquidity: U256,
    pub fee_rate: u32, // Basis points (e.g., 30 = 0.3%)
}

impl LiquidityPool {
    /// Create a new liquidity pool
    pub fn new(token_pair: TokenPair, fee_rate: u32) -> Self {
        Self {
            token_pair,
            reserve_a: U256::zero(),
            reserve_b: U256::zero(),
            total_liquidity: U256::zero(),
            fee_rate,
        }
    }

    /// Calculate the amount of token B received for a given amount of token A
    pub fn calculate_swap_output(&self, token_in: Address, amount_in: U256) -> Result<U256, DEXError> {
        if amount_in.is_zero() {
            return Err(DEXError::InvalidAmount("Amount in cannot be zero".to_string()));
        }

        let (reserve_in, reserve_out) = if token_in == self.token_pair.token_a {
            (&self.reserve_a, &self.reserve_b)
        } else if token_in == self.token_pair.token_b {
            (&self.reserve_b, &self.reserve_a)
        } else {
            return Err(DEXError::InvalidTokenPair("Token not in pool".to_string()));
        };

        if reserve_in.is_zero() || reserve_out.is_zero() {
            return Err(DEXError::InsufficientLiquidity("Insufficient liquidity in pool".to_string()));
        }

        // Calculate amount out with fee
        let fee_amount = (amount_in * U256::from(self.fee_rate)) / U256::from(10000);
        let amount_in_after_fee = amount_in - fee_amount;
        
        let numerator = amount_in_after_fee * *reserve_out;
        let denominator = *reserve_in + amount_in_after_fee;
        
        let amount_out = numerator / denominator;
        
        if amount_out.is_zero() {
            return Err(DEXError::InsufficientLiquidity("Amount out is zero".to_string()));
        }
        
        Ok(amount_out)
    }

    /// Calculate the amount of token A needed to get a specific amount of token B
    pub fn calculate_swap_input(&self, token_out: Address, amount_out: U256) -> Result<U256, DEXError> {
        if amount_out.is_zero() {
            return Err(DEXError::InvalidAmount("Amount out cannot be zero".to_string()));
        }

        if amount_out >= *self.reserve_b || amount_out >= *self.reserve_a {
            return Err(DEXError::InsufficientLiquidity("Insufficient liquidity in pool".to_string()));
        }

        let (reserve_in, reserve_out) = if token_out == self.token_pair.token_b {
            (&self.reserve_a, &self.reserve_b)
        } else if token_out == self.token_pair.token_a {
            (&self.reserve_b, &self.reserve_a)
        } else {
            return Err(DEXError::InvalidTokenPair("Token not in pool".to_string()));
        };

        let numerator = *reserve_in * amount_out * U256::from(10000);
        let denominator = (*reserve_out - amount_out) * U256::from(10000 - self.fee_rate);
        
        let amount_in = (numerator / denominator) + U256::one();
        
        Ok(amount_in)
    }

    /// Add liquidity to the pool
    pub fn add_liquidity(&mut self, amount_a: U256, amount_b: U256) -> Result<U256, DEXError> {
        if amount_a.is_zero() || amount_b.is_zero() {
            return Err(DEXError::InvalidAmount("Amounts must be greater than zero".to_string()));
        }

        let liquidity = if self.total_liquidity.is_zero() {
            // First liquidity provider
            (amount_a * amount_b).integer_sqrt()
        } else {
            // Calculate proportional liquidity
            let liquidity_a = (amount_a * self.total_liquidity) / self.reserve_a;
            let liquidity_b = (amount_b * self.total_liquidity) / self.reserve_b;
            std::cmp::min(liquidity_a, liquidity_b)
        };

        self.reserve_a = self.reserve_a + amount_a;
        self.reserve_b = self.reserve_b + amount_b;
        self.total_liquidity = self.total_liquidity + liquidity;

        Ok(liquidity)
    }

    /// Remove liquidity from the pool
    pub fn remove_liquidity(&mut self, liquidity: U256) -> Result<(U256, U256), DEXError> {
        if liquidity.is_zero() {
            return Err(DEXError::InvalidAmount("Liquidity must be greater than zero".to_string()));
        }

        if liquidity > self.total_liquidity {
            return Err(DEXError::InsufficientLiquidity("Insufficient liquidity".to_string()));
        }

        let amount_a = (liquidity * self.reserve_a) / self.total_liquidity;
        let amount_b = (liquidity * self.reserve_b) / self.total_liquidity;

        self.reserve_a = self.reserve_a - amount_a;
        self.reserve_b = self.reserve_b - amount_b;
        self.total_liquidity = self.total_liquidity - liquidity;

        Ok((amount_a, amount_b))
    }
}

/// Order side (buy/sell)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrderSide {
    Buy,
    Sell,
}

/// Order type (market/limit)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrderType {
    Market,
    Limit,
}

/// Order in the order book
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: String,
    pub user: Address,
    pub token_pair: TokenPair,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub price: U256,
    pub amount: U256,
    pub filled_amount: U256,
    pub timestamp: u64,
}

impl Order {
    pub fn new(
        id: String,
        user: Address,
        token_pair: TokenPair,
        side: OrderSide,
        order_type: OrderType,
        price: U256,
        amount: U256,
        timestamp: u64,
    ) -> Self {
        Self {
            id,
            user,
            token_pair,
            side,
            order_type,
            price,
            amount,
            filled_amount: U256::zero(),
            timestamp,
        }
    }

    pub fn remaining_amount(&self) -> U256 {
        self.amount - self.filled_amount
    }

    pub fn is_filled(&self) -> bool {
        self.filled_amount >= self.amount
    }

    pub fn fill_amount(&mut self, amount: U256) {
        self.filled_amount = std::cmp::min(self.filled_amount + amount, self.amount);
    }
}

/// Order book for a token pair
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBook {
    pub token_pair: TokenPair,
    pub buy_orders: Vec<Order>,
    pub sell_orders: Vec<Order>,
}

impl OrderBook {
    pub fn new(token_pair: TokenPair) -> Self {
        Self {
            token_pair,
            buy_orders: Vec::new(),
            sell_orders: Vec::new(),
        }
    }

    /// Add an order to the order book
    pub fn add_order(&mut self, order: Order) -> Result<(), DEXError> {
        if order.token_pair != self.token_pair {
            return Err(DEXError::InvalidTokenPair("Order token pair does not match order book".to_string()));
        }

        match order.side {
            OrderSide::Buy => {
                self.buy_orders.push(order);
                // Sort buy orders by price (highest first)
                self.buy_orders.sort_by(|a, b| b.price.cmp(&a.price));
            }
            OrderSide::Sell => {
                self.sell_orders.push(order);
                // Sort sell orders by price (lowest first)
                self.sell_orders.sort_by(|a, b| a.price.cmp(&b.price));
            }
        }

        Ok(())
    }

    /// Match orders in the order book
    pub fn match_orders(&mut self) -> Vec<(Order, Order, U256, U256)> {
        let mut matches = Vec::new();
        
        let mut i = 0;
        while i < self.buy_orders.len() && !self.buy_orders[i].is_filled() {
            let mut j = 0;
            while j < self.sell_orders.len() && !self.sell_orders[j].is_filled() {
                let buy_order = &self.buy_orders[i];
                let sell_order = &self.sell_orders[j];
                
                // Check if orders can be matched
                if buy_order.price >= sell_order.price {
                    let matched_amount = std::cmp::min(
                        buy_order.remaining_amount(),
                        sell_order.remaining_amount(),
                    );
                    
                    let matched_price = sell_order.price; // Use seller's price
                    
                    if !matched_amount.is_zero() {
                        // Create matched orders
                        let mut matched_buy = buy_order.clone();
                        let mut matched_sell = sell_order.clone();
                        
                        matched_buy.fill_amount(matched_amount);
                        matched_sell.fill_amount(matched_amount);
                        
                        matches.push((matched_buy, matched_sell, matched_amount, matched_price));
                        
                        // Update the original orders
                        self.buy_orders[i].fill_amount(matched_amount);
                        self.sell_orders[j].fill_amount(matched_amount);
                    }
                }
                
                // Move to next sell order if current one is filled
                if self.sell_orders[j].is_filled() {
                    j += 1;
                } else {
                    break;
                }
            }
            
            // Move to next buy order if current one is filled
            if self.buy_orders[i].is_filled() {
                i += 1;
            } else {
                break;
            }
        }
        
        // Remove filled orders
        self.buy_orders.retain(|order| !order.is_filled());
        self.sell_orders.retain(|order| !order.is_filled());
        
        matches
    }

    /// Cancel an order by ID
    pub fn cancel_order(&mut self, order_id: &str) -> Result<Option<Order>, DEXError> {
        // Check buy orders
        if let Some(pos) = self.buy_orders.iter().position(|order| order.id == order_id) {
            return Ok(Some(self.buy_orders.remove(pos)));
        }
        
        // Check sell orders
        if let Some(pos) = self.sell_orders.iter().position(|order| order.id == order_id) {
            return Ok(Some(self.sell_orders.remove(pos)));
        }
        
        Ok(None)
    }

    /// Get the best bid price (highest buy order)
    pub fn best_bid(&self) -> Option<U256> {
        self.buy_orders.first().map(|order| order.price)
    }

    /// Get the best ask price (lowest sell order)
    pub fn best_ask(&self) -> Option<U256> {
        self.sell_orders.first().map(|order| order.price)
    }
}

/// DEX Core service
pub struct DEXCore {
    liquidity_pools: std::collections::HashMap<TokenPair, LiquidityPool>,
    order_books: std::collections::HashMap<TokenPair, OrderBook>,
    tokens: std::collections::HashMap<Address, Token>,
    key_manager: KeyManager,
    signature_verifier: SignatureVerifier,
    authorization: Authorization,
}

impl DEXCore {
    /// Create a new DEX Core service
    pub fn new() -> Self {
        Self {
            liquidity_pools: std::collections::HashMap::new(),
            order_books: std::collections::HashMap::new(),
            tokens: std::collections::HashMap::new(),
            key_manager: KeyManager::new(),
            signature_verifier: SignatureVerifier::new(),
            authorization: Authorization::new(),
        }
    }

    /// Add a token to the DEX
    pub fn add_token(&mut self, token: Token) {
        self.tokens.insert(token.address, token);
    }

    /// Get token information
    pub fn get_token(&self, address: Address) -> Option<&Token> {
        self.tokens.get(&address)
    }

    /// Create a liquidity pool
    pub fn create_liquidity_pool(&mut self, token_a: Address, token_b: Address, fee_rate: u32) -> Result<(), DEXError> {
        let token_pair = TokenPair::new(token_a, token_b);
        
        if self.liquidity_pools.contains_key(&token_pair) {
            return Err(DEXError::LiquidityPoolError("Pool already exists".to_string()));
        }
        
        let pool = LiquidityPool::new(token_pair.clone(), fee_rate);
        self.liquidity_pools.insert(token_pair.clone(), pool);
        
        // Create corresponding order book
        if !self.order_books.contains_key(&token_pair) {
            let order_book = OrderBook::new(token_pair);
            self.order_books.insert(token_pair, order_book);
        }
        
        Ok(())
    }

    /// Add liquidity to a pool
    pub fn add_liquidity(&mut self, token_a: Address, token_b: Address, amount_a: U256, amount_b: U256) -> Result<U256, DEXError> {
        let token_pair = TokenPair::new(token_a, token_b);
        
        let pool = self.liquidity_pools.get_mut(&token_pair)
            .ok_or_else(|| DEXError::InvalidTokenPair("Pool does not exist".to_string()))?;
            
        let liquidity = pool.add_liquidity(amount_a, amount_b)?;
        Ok(liquidity)
    }

    /// Remove liquidity from a pool
    pub fn remove_liquidity(&mut self, token_a: Address, token_b: Address, liquidity: U256) -> Result<(U256, U256), DEXError> {
        let token_pair = TokenPair::new(token_a, token_b);
        
        let pool = self.liquidity_pools.get_mut(&token_pair)
            .ok_or_else(|| DEXError::InvalidTokenPair("Pool does not exist".to_string()))?;
            
        let amounts = pool.remove_liquidity(liquidity)?;
        Ok(amounts)
    }

    /// Swap tokens using AMM
    pub fn swap_tokens(&mut self, token_in: Address, token_out: Address, amount_in: U256, min_amount_out: U256) -> Result<U256, DEXError> {
        let token_pair = TokenPair::new(token_in, token_out);
        
        let pool = self.liquidity_pools.get_mut(&token_pair)
            .ok_or_else(|| DEXError::InvalidTokenPair("Pool does not exist".to_string()))?;
            
        let amount_out = pool.calculate_swap_output(token_in, amount_in)?;
        
        if amount_out < min_amount_out {
            return Err(DEXError::InsufficientLiquidity(
                format!("Amount out {} is less than minimum required {}", amount_out, min_amount_out)
            ));
        }
        
        // Update reserves
        if token_in == token_pair.token_a {
            pool.reserve_a = pool.reserve_a + amount_in;
            pool.reserve_b = pool.reserve_b - amount_out;
        } else {
            pool.reserve_b = pool.reserve_b + amount_in;
            pool.reserve_a = pool.reserve_a - amount_out;
        }
        
        Ok(amount_out)
    }

    /// Place an order in the order book
    pub fn place_order(&mut self, order: Order) -> Result<(), DEXError> {
        let order_book = self.order_books.get_mut(&order.token_pair)
            .ok_or_else(|| DEXError::OrderBookError("Order book does not exist".to_string()))?;
            
        order_book.add_order(order)?;
        Ok(())
    }

    /// Cancel an order
    pub fn cancel_order(&mut self, token_pair: TokenPair, order_id: &str) -> Result<Option<Order>, DEXError> {
        let order_book = self.order_books.get_mut(&token_pair)
            .ok_or_else(|| DEXError::OrderBookError("Order book does not exist".to_string()))?;
            
        order_book.cancel_order(order_id)
    }

    /// Match orders in an order book
    pub fn match_orders(&mut self, token_pair: TokenPair) -> Result<Vec<(Order, Order, U256, U256)>, DEXError> {
        let order_book = self.order_books.get_mut(&token_pair)
            .ok_or_else(|| DEXError::OrderBookError("Order book does not exist".to_string()))?;
            
        Ok(order_book.match_orders())
    }

    /// Get liquidity pool information
    pub fn get_liquidity_pool(&self, token_a: Address, token_b: Address) -> Option<&LiquidityPool> {
        let token_pair = TokenPair::new(token_a, token_b);
        self.liquidity_pools.get(&token_pair)
    }

    /// Get order book information
    pub fn get_order_book(&self, token_pair: TokenPair) -> Option<&OrderBook> {
        self.order_books.get(&token_pair)
    }

    /// Get all token pairs with liquidity
    pub fn get_active_pairs(&self) -> Vec<TokenPair> {
        self.liquidity_pools.keys().cloned().collect()
    }
}

/// DEX Core service initialization
pub fn init() -> Result<DEXCore, DEXError> {
    info!("DEX Core service initialized");
    Ok(DEXCore::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_token_pair_ordering() {
        let token_a = Address::from_str("0x1f9840a85d5af5bf1d1762f925bdaddc4201f984").unwrap();
        let token_b = Address::from_str("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48").unwrap();
        
        // Test consistent ordering
        let pair1 = TokenPair::new(token_a, token_b);
        let pair2 = TokenPair::new(token_b, token_a);
        
        assert_eq!(pair1, pair2);
        assert_eq!(pair1.token_a, pair2.token_a);
        assert_eq!(pair1.token_b, pair2.token_b);
    }

    #[test]
    fn test_liquidity_pool_creation() {
        let token_a = Address::from_str("0x1f9840a85d5af5bf1d1762f925bdaddc4201f984").unwrap();
        let token_b = Address::from_str("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48").unwrap();
        let token_pair = TokenPair::new(token_a, token_b);
        
        let pool = LiquidityPool::new(token_pair, 30); // 0.3% fee
        
        assert_eq!(pool.reserve_a, U256::zero());
        assert_eq!(pool.reserve_b, U256::zero());
        assert_eq!(pool.total_liquidity, U256::zero());
        assert_eq!(pool.fee_rate, 30);
    }

    #[test]
    fn test_liquidity_pool_swap_output() {
        let token_a = Address::from_str("0x1f9840a85d5af5bf1d1762f925bdaddc4201f984").unwrap();
        let token_b = Address::from_str("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48").unwrap();
        let token_pair = TokenPair::new(token_a, token_b);
        
        let mut pool = LiquidityPool::new(token_pair, 30); // 0.3% fee
        pool.reserve_a = U256::from(1000000u64); // 1M tokens
        pool.reserve_b = U256::from(1000000u64); // 1M tokens
        
        // Test swap calculation
        let amount_in = U256::from(1000u64); // 1000 tokens in
        let amount_out = pool.calculate_swap_output(token_a, amount_in).unwrap();
        
        // With 0.3% fee, we should get slightly less than 1000 tokens out
        assert!(amount_out < U256::from(1000u64));
        assert!(amount_out > U256::from(990u64)); // Should be around 997 after 0.3% fee
    }

    #[test]
    fn test_liquidity_pool_add_remove_liquidity() {
        let token_a = Address::from_str("0x1f9840a85d5af5bf1d1762f925bdaddc4201f984").unwrap();
        let token_b = Address::from_str("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48").unwrap();
        let token_pair = TokenPair::new(token_a, token_b);
        
        let mut pool = LiquidityPool::new(token_pair, 30);
        
        // Add initial liquidity
        let liquidity1 = pool.add_liquidity(U256::from(1000u64), U256::from(2000u64)).unwrap();
        assert!(liquidity1 > U256::zero());
        assert_eq!(pool.reserve_a, U256::from(1000u64));
        assert_eq!(pool.reserve_b, U256::from(2000u64));
        
        // Add more liquidity
        let liquidity2 = pool.add_liquidity(U256::from(500u64), U256::from(1000u64)).unwrap();
        assert!(liquidity2 > U256::zero());
        assert_eq!(pool.reserve_a, U256::from(1500u64));
        assert_eq!(pool.reserve_b, U256::from(3000u64));
        
        // Remove liquidity
        let (amount_a, amount_b) = pool.remove_liquidity(liquidity1).unwrap();
        assert!(amount_a > U256::zero());
        assert!(amount_b > U256::zero());
    }

    #[test]
    fn test_order_creation() {
        let token_a = Address::from_str("0x1f9840a85d5af5bf1d1762f925bdaddc4201f984").unwrap();
        let token_b = Address::from_str("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48").unwrap();
        let token_pair = TokenPair::new(token_a, token_b);
        let user = Address::from_str("0xd8da6bf26964af9d7eed9e03e53415d37aa96045").unwrap();
        
        let order = Order::new(
            "order1".to_string(),
            user,
            token_pair,
            OrderSide::Buy,
            OrderType::Limit,
            U256::from(100u64),
            U256::from(500u64),
            1234567890,
        );
        
        assert_eq!(order.id, "order1");
        assert_eq!(order.user, user);
        assert_eq!(order.side, OrderSide::Buy);
        assert_eq!(order.order_type, OrderType::Limit);
        assert_eq!(order.price, U256::from(100u64));
        assert_eq!(order.amount, U256::from(500u64));
        assert_eq!(order.filled_amount, U256::zero());
        assert_eq!(order.timestamp, 1234567890);
    }

    #[test]
    fn test_order_book_matching() {
        let token_a = Address::from_str("0x1f9840a85d5af5bf1d1762f925bdaddc4201f984").unwrap();
        let token_b = Address::from_str("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48").unwrap();
        let token_pair = TokenPair::new(token_a, token_b);
        let user1 = Address::from_str("0xd8da6bf26964af9d7eed9e03e53415d37aa96045").unwrap();
        let user2 = Address::from_str("0x5a1f8e37ea381e1121854199126353b327f02b32").unwrap();
        
        let mut order_book = OrderBook::new(token_pair.clone());
        
        // Add buy order
        let buy_order = Order::new(
            "buy1".to_string(),
            user1,
            token_pair.clone(),
            OrderSide::Buy,
            OrderType::Limit,
            U256::from(100u64),
            U256::from(500u64),
            1234567890,
        );
        
        // Add sell order
        let sell_order = Order::new(
            "sell1".to_string(),
            user2,
            token_pair,
            OrderSide::Sell,
            OrderType::Limit,
            U256::from(95u64),
            U256::from(500u64),
            1234567891,
        );
        
        order_book.add_order(buy_order).unwrap();
        order_book.add_order(sell_order).unwrap();
        
        // Match orders
        let matches = order_book.match_orders();
        assert_eq!(matches.len(), 1);
        
        let (matched_buy, matched_sell, amount, price) = &matches[0];
        assert_eq!(*amount, U256::from(500u64));
        assert_eq!(*price, U256::from(95u64)); // Seller's price
        assert_eq!(matched_buy.id, "buy1");
        assert_eq!(matched_sell.id, "sell1");
    }

    #[test]
    fn test_dex_core_initialization() {
        let dex = DEXCore::new();
        assert_eq!(dex.liquidity_pools.len(), 0);
        assert_eq!(dex.order_books.len(), 0);
        assert_eq!(dex.tokens.len(), 0);
    }

    #[test]
    fn test_dex_core_token_management() {
        let mut dex = DEXCore::new();
        let token_address = Address::from_str("0x1f9840a85d5af5bf1d1762f925bdaddc4201f984").unwrap();
        let token = Token {
            address: token_address,
            name: "Test Token".to_string(),
            symbol: "TST".to_string(),
            decimals: 18,
        };
        
        dex.add_token(token.clone());
        let retrieved_token = dex.get_token(token_address).unwrap();
        assert_eq!(retrieved_token.symbol, "TST");
    }

    #[test]
    fn test_dex_core_liquidity_pool_operations() {
        let mut dex = DEXCore::new();
        let token_a = Address::from_str("0x1f9840a85d5af5bf1d1762f925bdaddc4201f984").unwrap();
        let token_b = Address::from_str("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48").unwrap();
        
        // Create pool
        let result = dex.create_liquidity_pool(token_a, token_b, 30);
        assert!(result.is_ok());
        
        // Add liquidity
        let liquidity = dex.add_liquidity(token_a, token_b, U256::from(1000u64), U256::from(2000u64));
        assert!(liquidity.is_ok());
        
        // Get pool
        let pool = dex.get_liquidity_pool(token_a, token_b).unwrap();
        assert_eq!(pool.reserve_a, U256::from(1000u64));
        assert_eq!(pool.reserve_b, U256::from(2000u64));
    }

    #[test]
    fn test_dex_core_swap_tokens() {
        let mut dex = DEXCore::new();
        let token_a = Address::from_str("0x1f9840a85d5af5bf1d1762f925bdaddc4201f984").unwrap();
        let token_b = Address::from_str("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48").unwrap();
        
        // Create pool and add liquidity
        dex.create_liquidity_pool(token_a, token_b, 30).unwrap();
        dex.add_liquidity(token_a, token_b, U256::from(1000000u64), U256::from(1000000u64)).unwrap();
        
        // Perform swap
        let amount_out = dex.swap_tokens(token_a, token_b, U256::from(1000u64), U256::from(900u64));
        assert!(amount_out.is_ok());
        assert!(amount_out.unwrap() > U256::from(900u64));
    }
}