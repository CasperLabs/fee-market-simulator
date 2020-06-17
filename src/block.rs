use crate::transaction::Transaction;
use stats::{mean, median};

pub struct Block {
    txs: Vec<Transaction>,
    gas_limit: u64,
}

impl Block {
    fn new(gas_limit: u64) -> Block {
        Block {
            txs: Vec::new(),
            gas_limit: gas_limit,
        }
    }

    fn add_txs(&mut self, txs: Vec<Transaction>) {
        self.txs.extend(txs.iter().cloned())
    }

    fn get_gas_used(&self) -> u64 {
        self.txs.iter().map(|x| x.gas_used).sum()
    }

    fn get_fullness(&self) -> u64 {
        self.get_gas_used() / self.gas_limit
    }

    fn get_median_price(&self) -> u64 {
        median(self.txs.iter().map(|x| x.gas_price)).unwrap() as u64
    }

    fn get_mean_price(&self) -> u64 {
        mean(self.txs.iter().map(|x| x.gas_price)) as u64
    }

    fn get_min_price(&self) -> u64 {
        self.txs.iter().map(|x| x.gas_price).min().unwrap()
    }

    fn get_max_price(&self) -> u64 {
        self.txs.iter().map(|x| x.gas_price).max().unwrap()
    }
}
