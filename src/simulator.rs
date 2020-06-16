use crate::demand::DemandCurve;
use crate::transaction::TransactionPool;
use crate::block::Block;

pub struct FeeMarketSimulator<'a> {
    demand_curve: DemandCurve,
    initial_price: u64,
    output_dir: &'a str,
    block_gas_limit: u64,
    tx_gas_used: u64,
    block_time: u64,
    control_range: u64,
    target_fullness: f64,
    price_adjustment_rate: f64,
    //
    txpool: TransactionPool,
    time_arr: Vec<u64>,
    n_user_arr: Vec<u64>,
    txpool_size_arr: Vec<u64>,
    txs_sent_arr: Vec<u64>,
    blocks: Vec<Block>,
    control_fullness_arr: Vec<f64>,
    fixed_price_arr: Vec<u64>,
    n_unincluded_tx_arr: Vec<u64>,
}


impl<'a> FeeMarketSimulator<'_> {
    fn new_price_adjustment_simulator(
        demand_curve: DemandCurve,
        initial_price: u64,
        output_dir: &'a str,
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
            output_dir: output_dir,
            block_gas_limit: block_gas_limit,
            tx_gas_used: tx_gas_used,
            block_time: block_time,
            control_range: control_range,
            target_fullness: target_fullness,
            price_adjustment_rate: price_adjustment_rate,
            //
            txpool: TransactionPool::new(txpool_size),
            time_arr: Vec::new(),
            n_user_arr: Vec::new(),
            txpool_size_arr: Vec::new(),
            txs_sent_arr: Vec::new(),
            blocks: Vec::new(),
            control_fullness_arr: Vec::new(),
            fixed_price_arr: Vec::new(),
            n_unincluded_tx_arr: Vec::new(),
        }
    }

    fn run(&mut self, n_user_vec: Vec<u64>) {

    }
}
