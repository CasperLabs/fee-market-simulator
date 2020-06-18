use indicatif::ProgressBar;
use stats::{mean, median};
use std::env::{current_dir, join_paths};
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use crate::block::Block;
use crate::demand::DemandCurve;
use crate::transaction::{Transaction, TransactionPool};

pub struct FeeMarketSimulator {
    demand_curve: DemandCurve,
    initial_price: u64,
    block_gas_limit: u64,
    tx_gas_used: u64,
    block_time: u64,
    control_range: u64,
    target_fullness: f64,
    price_adjustment_rate: f64,
    //
    txpool: TransactionPool,
    blocks: Vec<Block>,
    // time_vec: Vec<u64>,
    // n_user_vec: Vec<u64>,
    // txpool_size_vec: Vec<u64>,
    // txs_sent_vec: Vec<u64>,
    // control_fullness_vec: Vec<f64>,
    // fixed_price_vec: Vec<u64>,
    // n_unincluded_tx_vec: Vec<u64>,
}

impl FeeMarketSimulator {
    pub fn new_autoprice_simulator(
        demand_curve: DemandCurve,
        initial_price: u64,
        block_gas_limit: u64,
        tx_gas_used: u64,
        txpool_size: usize,
        block_time: u64,
        control_range: u64,
        target_fullness: f64,
        price_adjustment_rate: f64,
    ) -> FeeMarketSimulator {
        FeeMarketSimulator {
            demand_curve: demand_curve,
            initial_price: initial_price,
            block_gas_limit: block_gas_limit,
            tx_gas_used: tx_gas_used,
            block_time: block_time,
            control_range: control_range,
            target_fullness: target_fullness,
            price_adjustment_rate: price_adjustment_rate,
            //
            txpool: TransactionPool::new(txpool_size),
            blocks: Vec::new(),
            // time_vec: Vec::new(),
            // n_user_vec: Vec::new(),
            // txpool_size_vec: Vec::new(),
            // txs_sent_vec: Vec::new(),
            // control_fullness_vec: Vec::new(),
            // fixed_price_vec: Vec::new(),
            // n_unincluded_tx_vec: Vec::new(),
        }
    }

    pub fn run(&mut self, n_user_vec: Vec<u64>, output_dir: PathBuf) {
        let mut output_csv_path = output_dir.clone();
        output_csv_path.push("out.csv");

        fs::create_dir_all(output_dir);

        let mut output_csv_file = File::create(output_csv_path).unwrap();

        output_csv_file.write_all("height,time,n_user,n_sent_tx,n_included_tx,n_unincluded_tx,txpool_size,control_fullness,fixed_price\n".as_bytes()).unwrap();

        let bar = ProgressBar::new(n_user_vec.len() as u64);

        let mut fixed_price: u64 = self.initial_price;
        let count = 0;
        let mut control_fullness: f64 = 0.;

        for x_ in 0..n_user_vec.len() {
            let x = x_ as u64;
            let time = x * self.block_time;
            let n_user = n_user_vec[x_];

            if x > 0 && x % self.control_range == 0 {
                let control_blocks: Vec<&Block> = self
                    .blocks
                    .iter()
                    .rev()
                    .take(self.control_range as usize)
                    .collect();

                // let control_gas_used: u64 = control_blocks.iter().map(|&b| b.get_gas_used()).sum();
                // let max_gas_used = control_blocks.len() as u64 * self.block_gas_limit;
                // control_fullness = control_gas_used as f64 / max_gas_used as f64;

                control_fullness = median(control_blocks.iter().map(|&b| b.get_fullness()))
                    .expect("No blocks in the control range");

                let increase = control_fullness > self.target_fullness;

                if increase {
                    fixed_price = (fixed_price as f64 * (1. + self.price_adjustment_rate)) as u64;
                } else {
                    fixed_price = (fixed_price as f64 / (1. + self.price_adjustment_rate)) as u64;
                }

                // println!("{} {}", fixed_price, control_fullness);
            }

            let wtp_vec = self.demand_curve.sample_price(n_user as usize);

            // println!("{} {}", n_user, wtp_vec.len());
            let n_sent_tx = wtp_vec.iter().filter(|&&x| x >= fixed_price).count() as u64;
            let txs = (0..n_sent_tx)
                .map(|x| Transaction::new(self.tx_gas_used, fixed_price))
                .collect();

            self.txpool.add_txs(txs);
            // println!("{}", n_sent_tx);

            let included_txs = self.txpool.pop_most_valuable_txs(self.block_gas_limit);

            let mut new_block = Block::new(self.block_gas_limit);
            new_block.add_txs(included_txs);

            let n_included_tx = new_block.get_n_tx();
            let n_unincluded_tx = n_sent_tx.saturating_sub(n_included_tx);

            self.blocks.push(new_block);

            output_csv_file
                .write_all(
                    format!(
                        "{},{},{},{},{},{},{},{},{}\n",
                        x,
                        x * self.block_time,
                        n_user,             // number of users in the market
                        n_sent_tx,          // transactions sent
                        n_included_tx,      // number of transactions included in the block
                        n_unincluded_tx, // number of transactions sent but not included in the block
                        self.txpool.size(), // size of the transaction pool
                        control_fullness,
                        fixed_price
                    )
                    .as_bytes(),
                )
                .unwrap();

            bar.inc(1);
        }

        bar.finish();
    }
}
