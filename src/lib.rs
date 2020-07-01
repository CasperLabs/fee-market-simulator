pub mod block;
pub mod demand;
pub mod helper;
pub mod simulator;
pub mod sorted_list;
pub mod transaction;

pub use crate::simulator::FeeMarketSimulator;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
