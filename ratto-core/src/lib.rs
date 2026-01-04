#![cfg_attr(not(test), no_std)]

pub mod boot;
pub mod cpu;
pub mod mem;
pub mod sync;

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
