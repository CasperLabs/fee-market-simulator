use crate::transaction::Transaction;
use stats::{mean, median};

pub struct Block {
    txs: Vec<Transaction>,
    gas_limit: u64,
}

impl Block {
    pub fn new(gas_limit: u64) -> Block {
        Block {
            txs: Vec::new(),
            gas_limit: gas_limit,
        }
    }

    pub fn add_txs(&mut self, txs: Vec<Transaction>) {
        self.txs.extend(txs.iter().cloned())
    }

    pub fn gas_used(&self) -> u64 {
        self.txs.iter().map(Transaction::gas_used).sum()
    }

    pub fn fullness(&self) -> f64 {
        self.gas_used() as f64 / self.gas_limit as f64
    }

    pub fn median_price(&self) -> u64 {
        median(self.txs.iter().map(Transaction::gas_price)).unwrap() as u64
    }

    pub fn mean_price(&self) -> u64 {
        mean(self.txs.iter().map(Transaction::gas_price)) as u64
    }

    pub fn min_price(&self) -> u64 {
        self.txs.iter().map(Transaction::gas_price).min().unwrap()
    }

    pub fn max_price(&self) -> u64 {
        self.txs.iter().map(Transaction::gas_price).max().unwrap()
    }

    pub fn tx_count(&self) -> u64 {
        self.txs.len() as u64
    }
}
