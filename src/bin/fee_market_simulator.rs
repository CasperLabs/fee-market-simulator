use config;
use csv;
use clap::{App, Arg};
use std::fs::File;

use fee_market_simulator::demand::*;
use fee_market_simulator::helper::*;
use fee_market_simulator::transaction::*;

use fee_market_simulator::FeeMarketSimulator;
use std::collections::HashMap;

fn read_demand_profile(path: &str) -> Vec<u64> {
    let file = File::open(path).expect("Couldn't open input CSV file");
    let mut reader = csv::ReaderBuilder::new().has_headers(false).from_reader(file);

    let mut result: Vec<u64> = Vec::new();

    for record in reader.records() {
        let record = record.unwrap();
        result.push(record[0].parse().unwrap());
    }
    result
}

fn main() {
    let matches = App::new("Fee Market Simulator")
        .version("0.1")
        .author("Onur Solmaz <onursolmaz@gmail.com>")
        .arg(Arg::with_name("config")
             .short("c")
             .long("config")
             .value_name("FILE")
             .help("Path to the simulator config file")
             .takes_value(true))
        .get_matches();

    let config_path = matches.value_of("config").unwrap();

    let mut settings_ = config::Config::default();
    settings_.merge(config::File::with_name(config_path)).unwrap();

    let settings = settings_.try_into::<HashMap<String, String>>().unwrap();

    println!("{:?}", settings);

    // let mut a = TransactionPool::new(TXPOOL_SIZE);
    // let t1 = Transaction::new(1000, 10);
    // let t2 = Transaction::new(1000, 5);
    // a.add_txs(vec![t1, t2]);
    // a.pool.remove(1);
    // for i in a.pool {
    //     println!("{:?}", i);
    // }

    let mut dc = DemandCurve::from_csv(&settings["demand_curve_path"], settings["interp_resolution"].parse().unwrap());

    let mut sim = FeeMarketSimulator::new_price_adjustment_simulator(
        dc,
        settings["initial_price"].parse().unwrap(),
        &settings["output_dir"],
        settings["block_gas_limit"].parse().unwrap(),
        settings["tx_gas_used"].parse().unwrap(),
        settings["txpool_size"].parse().unwrap(),
        settings["block_time"].parse().unwrap(),
        settings["control_range"].parse().unwrap(),
        settings["target_fullness"].parse().unwrap(),
        settings["price_adjustment_rate"].parse().unwrap(),
    );

    let demand_profile = read_demand_profile(&settings["demand_profile_path"]);

    sim.run(demand_profile);

    // println!("{}", a.pool.len());

    // println!("{:?}", dc.sample_price(1000));

    // println!("{}", a.pool.len());
}
