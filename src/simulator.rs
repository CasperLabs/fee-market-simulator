use indicatif::ProgressBar;
use stats::median;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::iter;

use crate::block::Block;
use crate::demand::DemandCurve;
use crate::helper::LinearInterpolator;
use crate::transaction::{Transaction, TransactionPool};

pub struct FeeMarketSimulator {
    demand_curve: DemandCurve,
    token_price: Option<LinearInterpolator>,
    initial_price: u64,
    block_gas_limit: u64,
    tx_gas_used: u64,
    block_time: u64,
    control_range: u64,
    target_fullness: f64,
    price_adjustment_rate: f64,
    txpool: TransactionPool,
    blocks: Vec<Block>,
}

impl FeeMarketSimulator {
    pub fn new_autoprice_simulator(
        demand_curve: DemandCurve,
        token_price: Option<LinearInterpolator>,
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
            demand_curve,
            token_price,
            initial_price,
            block_gas_limit,
            tx_gas_used,
            block_time,
            control_range,
            target_fullness,
            price_adjustment_rate,
            txpool: TransactionPool::new(txpool_size),
            blocks: Vec::new(),
        }
    }

    pub fn run(&mut self, n_user_vec: Vec<u64>, output_dir: PathBuf) {
        let mut output_csv_path = output_dir.clone();
        output_csv_path.push("out.csv");

        let _ = fs::create_dir_all(output_dir).expect("Could not create the output directory");

        let mut output_csv_file = File::create(output_csv_path).unwrap();

        output_csv_file.write_all("height,time,n_user,n_sent_tx,n_included_tx,n_unincluded_tx,txpool_size,control_fullness,token_price,fixed_gas_price\n".as_bytes()).unwrap();

        let bar = ProgressBar::new(n_user_vec.len() as u64);

        let mut fixed_gas_price: u64 = self.initial_price;
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

                // let control_gas_used: u64 = control_blocks.iter().map(|&b| b.gas_used()).sum();
                // let max_gas_used = control_blocks.len() as u64 * self.block_gas_limit;
                // control_fullness = control_gas_used as f64 / max_gas_used as f64;

                control_fullness = median(control_blocks.into_iter().map(Block::fullness))
                    .expect("No blocks in the control range");

                let increase = control_fullness > self.target_fullness;

                if increase {
                    fixed_gas_price =
                        (fixed_gas_price as f64 * (1. + self.price_adjustment_rate)) as u64;
                } else {
                    fixed_gas_price =
                        (fixed_gas_price as f64 / (1. + self.price_adjustment_rate)) as u64;
                }

                // println!("{} {}", fixed_gas_price, control_fullness);
            }

            let wtp_vec = self.demand_curve.sample_price(n_user as usize);

            // println!("{} {}", n_user, wtp_vec.len());

            let mut current_token_price: f64 = 1.;

            let n_sent_tx: u64 = match &self.token_price {
                Some(interp) => {
                    // println!("{}", time);
                    let relative_time = interp.xmin() + time as f64;

                    if relative_time > interp.xmax() {
                        println!("Token price data not large enough to cover the whole simulation, exiting...");
                        break;
                    }
                    // token/fiat * gas/token = gas/fiat
                    current_token_price = interp.interpolate(relative_time);

                    wtp_vec
                        .iter()
                        .filter(|&&x| x as f64 >= fixed_gas_price as f64 * current_token_price)
                        .count() as u64
                }
                None => wtp_vec.iter().filter(|&&x| x >= fixed_gas_price).count() as u64,
            };

            // let txs = iter::repeat(Transaction::new(self.tx_gas_used, fixed_gas_price))
            //     .take(n_sent_tx as usize)
            //     .collect();

            let txs = (0..n_sent_tx).map(|x| Transaction::new(self.tx_gas_used, fixed_gas_price)).collect();

            self.txpool.add_txs(txs);

            let included_txs = self.txpool.pop_most_valuable_txs(self.block_gas_limit);

            let mut new_block = Block::new(self.block_gas_limit);
            new_block.add_txs(included_txs);

            let n_included_tx = new_block.tx_count();
            let n_unincluded_tx = n_sent_tx.saturating_sub(n_included_tx);

            self.blocks.push(new_block);

            output_csv_file
                .write_all(
                    format!(
                        "{},{},{},{},{},{},{},{},{},{}\n",
                        x,
                        x * self.block_time,
                        n_user,             // number of users in the market
                        n_sent_tx,          // transactions sent
                        n_included_tx,      // number of transactions included in the block
                        n_unincluded_tx, // number of transactions sent but not included in the block
                        self.txpool.size(), // size of the transaction pool
                        control_fullness,
                        current_token_price,
                        fixed_gas_price
                    )
                    .as_bytes(),
                )
                .unwrap();

            bar.inc(1);
        }

        bar.finish();
    }
}
