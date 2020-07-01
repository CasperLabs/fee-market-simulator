// use std::collections::BTreeMap;
use crate::sorted_list::SortedList;

static mut TX_COUNTER: u64 = 0;

#[derive(Clone, Copy, Debug)]
pub struct Transaction {
    id: u64,
    gas_used: u64,
    gas_price: u64,
}

impl Transaction {
    pub fn new(gas_used: u64, gas_price: u64) -> Transaction {
        unsafe {
            let result = Transaction {
                gas_used,
                gas_price,
                id: TX_COUNTER,
            };
            TX_COUNTER += 1;
            return result;
        }
    }

    pub fn fee(&self) -> u64 {
        self.gas_used * self.gas_price
    }

    pub fn gas_used(&self) -> u64 {
        self.gas_used
    }

    pub fn gas_price(&self) -> u64 {
        self.gas_price
    }
}

impl PartialEq for Transaction {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

pub struct TransactionPool {
    pool: SortedList<u64, Transaction>,
    limit: usize,
}

impl TransactionPool {
    pub fn new(limit: usize) -> TransactionPool {
        TransactionPool {
            pool: SortedList::new(),
            limit,
        }
    }

    pub fn add_txs(&mut self, txs: Vec<Transaction>) {
        for tx in txs {
            self.pool.insert(tx.gas_price, tx);
        }
    }

    pub fn pop_most_valuable_txs(&mut self, total_gas_target: u64) -> Vec<Transaction> {
        let mut result: Vec<Transaction> = Vec::new();
        let mut total_gas: u64 = 0;
        loop {
            if self.pool.len() == 0 {
                break;
            }
            if total_gas + (self.pool.get(self.pool.len() - 1)).gas_used > total_gas_target {
                break;
            }

            total_gas += self.pool.get(self.pool.len() - 1).gas_used;
            result.push(*(self.pool.get(self.pool.len() - 1)));
            self.pool.remove(self.pool.len() - 1);
        }
        result
    }

    pub fn size(&self) -> u64 {
        self.pool.len() as u64
    }
}
