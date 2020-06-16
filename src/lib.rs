#![feature(is_sorted)]

pub mod demand;
pub mod helper;
pub mod transaction;
pub mod sorted_list;
pub mod simulator;
pub mod block;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
