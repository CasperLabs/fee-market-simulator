// use std::collections::BTreeMap;
use crate::sorted_list::SortedList;

static mut TX_COUNTER: u64 = 0;

#[derive(Clone)]
#[derive(Debug)]
pub struct Transaction {
    pub id: u64,
    pub gas_used: u64,
    pub gas_price: u64,
}

impl Transaction {
    pub fn new(gas_used: u64, gas_price: u64) -> Transaction {
        unsafe {
            let result = Transaction {
                gas_used: gas_used,
                gas_price: gas_price,
                id: TX_COUNTER,
            };
            TX_COUNTER += 1;
            return result;
        }
    }
    fn get_fee(&self) -> u64 {
        self.gas_used * self.gas_price
    }
}

impl PartialEq for Transaction {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

pub struct TransactionPool {
    pub pool: SortedList<u64, Transaction>,
    limit: usize,
}

impl TransactionPool {
    pub fn new(limit: usize) -> TransactionPool {
        TransactionPool {
            pool: SortedList::new(),
            limit: limit,
        }
    }

    pub fn add_txs(&mut self, txs: Vec<Transaction>) {
        for tx in txs {
            self.pool.insert(tx.gas_price, tx);
        }
    }
}
