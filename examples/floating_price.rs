extern crate fee_market_simulator;
use fee_market_simulator::demand::*;
use fee_market_simulator::helper::*;
use fee_market_simulator::transaction::*;

// use fee_market_simulator;

const TXPOOL_SIZE: usize = 1_000_000;

fn main() {
    let mut a = TransactionPool::new(TXPOOL_SIZE);
    let t1 = Transaction::new(1000, 10);
    let t2 = Transaction::new(1000, 10);

    a.add_txs(vec![t1, t2]);
    println!("Hello world");

    // for i in a.pool {
    //     println!("{}", i.1.id);
    // }
    println!("{}", a.pool.len());

    let p = vec![0, 1000, 50000, 200000, 400000, 600000, 800000, 1000000];
    let q = vec![100000, 20000, 15000, 10000, 6000, 3000, 1000, 0];
    let mut dc = DemandCurve::new(p, q, 1000);

    println!("{:?}", dc.sample_price(1000));

    // a.pool.pop();

    // println!("{}", a.pool.len());
}
